use once_cell::sync::Lazy;
use regex::Regex;
use unicode_normalization::{char::is_combining_mark, UnicodeNormalization};

/// Lower‑case, ASCII‑fold, collapse inner whitespace and trim.
/// Also decodes a handful of HTML entities and normalises curly quotes into
/// their straight ASCII equivalents.  This helper performs the heavy
/// lifting for all of the top‑level normalisation routines.
fn fold_basic(s: &str) -> String {
    // 1. Canonical decomposition → remove diacritics
    // 2. Lower‑case
    // 3. Filter combining marks
    let mut decomposed = String::new();
    for c in s.nfd() {
        if !is_combining_mark(c) {
            for l in c.to_lowercase() {
                decomposed.push(l);
            }
        }
    }

    // 4. Replace HTML entities & typographic quotes/apostrophes we care about.
    // We perform these substitutions before collapsing whitespace so that
    // ampersands can subsequently be stripped by `strip_common_punct` and
    // curly quotes become plain ASCII and are then removed if needed.
    let replaced = decomposed
        .replace("&amp;", "&")
        .replace('“', "\"")
        .replace('”', "\"")
        .replace('‘', "'")
        .replace('’', "'");

    // 5. Collapse runs of whitespace into a single space and trim.
    let mut out = String::with_capacity(replaced.len());
    let mut last_space = false;
    for ch in replaced.chars() {
        if ch.is_whitespace() {
            if !last_space {
                out.push(' ');
                last_space = true;
            }
        } else {
            out.push(ch);
            last_space = false;
        }
    }
    out.trim().to_owned()
}

/// Remove punctuation characters that rarely matter in comparisons.
fn strip_common_punct(s: &str) -> String {
    s.chars()
        .filter(|c| !c.is_ascii_punctuation())
        .collect()
}

/// Song‑level decoration such as “(Remastered 2019)” or “- Live”.
static DECORATION_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)(?:[\(\[\{][^)\]\}]*?(?:remaster|live|version|edit|mix|karaoke|mono|instrumental|acoustic)[^)\]\}]*?[\)\]\}]|\s*[-–—]\s*(?:\d{2,4}\s*)?(?:live|remaster|version|edit|mix|karaoke|mono|instrumental|acoustic)(?:\s*\d{2,4})?\s*$)",
    )
    .unwrap()
});

/// Album‑level decoration such as “(Deluxe Edition)” or “– Anniversary Edition”.
static ALBUM_DECORATION_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)(?:[\(\[\{][^)\]\}]*?(?:deluxe|expanded|anniversary|remaster|edition|version)[^)\]\}]*?[\)\]\}]|\s*[-–—]\s*(?:(?:deluxe|expanded|anniversary|remaster|edition|version)(?:\s+(?:edition|version))?)\s*$)",
    )
    .unwrap()
});

/// “feat.” variants in titles.
static FEAT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)\s+\(?\s*(?:feat(?:\.|\b)|featuring|ft(?:\.|\b))\s+[^)]+\)?"
    )
    .unwrap()
});

/// Split characters that typically join multiple artists.
static ARTIST_SPLIT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)\s*(?:&| and | x |,|;|\+)\s*").unwrap());

/// Normalize a *song / track* title.
pub fn normalize_song_title(raw: &str) -> String {
    // Remove any (feat...), ft..., featuring... block
    let no_feat = FEAT_RE.replace_all(raw, "").to_string();
    // Remove decorations (remix/live/remaster etc.)
    let core = DECORATION_RE.replace_all(&no_feat, "").to_string();

    // First pass: decode entities, lower‑case, accent‑fold, collapse whitespace
    let base = fold_basic(&core);
    // Remove punctuation (e.g. & or quotes)
    let stripped = strip_common_punct(&base);
    // Second pass: collapse whitespace again (after punctuation removal)
    fold_basic(&stripped)
}

/// Normalize an *album* title.
pub fn normalize_album_title(raw: &str) -> String {
    // Strip typical album decorations (deluxe, expanded, etc.)
    let core = ALBUM_DECORATION_RE.replace_all(raw, "").to_string();
    let base = fold_basic(&core);
    let stripped = strip_common_punct(&base);
    fold_basic(&stripped)
}

/// Normalize an *artist* (or list of artists) string.
pub fn normalize_artist_name(raw: &str) -> String {
    // Base fold: lower‑case, remove accents, decode basic entities, collapse whitespace
    let basic = fold_basic(raw);

    // Replace dash‑like characters with spaces
    const DASHES: [char; 6] = ['-', '‐', '‒', '–', '—', '−'];
    let mut dash_as_space = String::with_capacity(basic.len());
    for ch in basic.chars() {
        if DASHES.contains(&ch) {
            dash_as_space.push(' ');
        } else {
            dash_as_space.push(ch);
        }
    }

    // Standardise connectors to " and "
    let connectors_standardised = ARTIST_SPLIT_RE
        .replace_all(&dash_as_space, " and ")
        .to_string();

    // Remove any remaining ASCII punctuation (e.g. slash in AC/DC)
    let cleaned = strip_common_punct(&connectors_standardised);

    // Split on " and ", trim tokens, drop "various artists", sort, deduplicate
    let mut parts: Vec<_> = cleaned
        .split(" and ")
        .map(str::trim)
        .filter(|s| !s.is_empty() && *s != "various artists")
        .collect();

    parts.sort_unstable();
    parts.dedup();
    parts.join(" and ")
}
/// --------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn track_examples() {
        assert_eq!(
            normalize_song_title("Shape of You (Extended Version)"),
            "shape of you"
        );
        assert_eq!(
            normalize_song_title("Hôtel California – Live 1999"),
            "hotel california"
        );
    }

    #[test]
    fn track_feat_and_decorations() {
        // Strip feat./ft./featuring, with and without parentheses
        assert_eq!(
            normalize_song_title("Blinding Lights (feat. ROSALÍA)"),
            "blinding lights"
        );
        assert_eq!(
            normalize_song_title("Blinding Lights feat. ROSALIA"),
            "blinding lights"
        );
        assert_eq!(
            normalize_song_title("Blinding Lights ft. Rosalia"),
            "blinding lights"
        );
        assert_eq!(
            normalize_song_title("Blinding Lights (featuring Rosalia)"),
            "blinding lights"
        );

        // Remove various decorations
        assert_eq!(
            normalize_song_title("Song Name (Remastered 2014)"),
            "song name"
        );
        assert_eq!(
            normalize_song_title("Song Name [Acoustic Version]"),
            "song name"
        );
        assert_eq!(
            normalize_song_title("Song Name – Live"),
            "song name"
        );
        assert_eq!(
            normalize_song_title("Song Name – Live 2001"),
            "song name"
        );
        assert_eq!(
            normalize_song_title("Song Name - Mix"),
            "song name"
        );

        // HTML entity & quotes, punctuation stripping, whitespace collapse
        assert_eq!(normalize_song_title("Rock &amp; Roll"), "rock roll");
        assert_eq!(normalize_song_title("“Hello”"), "hello");
        assert_eq!(normalize_song_title("  Don't   Stop  (Remaster) "), "dont stop");
    }

    #[test]
    fn album_examples() {
        assert_eq!(
            normalize_album_title("Back to Black (Deluxe Edition)"),
            "back to black"
        );
    }

    #[test]
    fn album_more_cases() {
        // Keep EP/Single labels that are part of the main title
        assert_eq!(normalize_album_title("Random Title - EP"), "random title ep");
        assert_eq!(normalize_album_title("Random Title - Single"), "random title single");

        // Strip typical album-level decorations
        assert_eq!(
            normalize_album_title("Album (Remastered 2011) [Deluxe Edition]"),
            "album"
        );
        assert_eq!(
            normalize_album_title("Album – Anniversary Edition"),
            "album"
        );
    }

    #[test]
    fn artist_examples() {
        assert_eq!(normalize_artist_name("Beyoncé & JAY-Z"), "beyonce and jay z");
        assert_eq!(normalize_artist_name("AC/DC"), "acdc");
    }

    #[test]
    fn artist_connectors_sort_and_dedup() {
        // Mix of connectors; tokens sorted & deduped
        assert_eq!(
            normalize_artist_name("Beyoncé x JAY-Z & Ed Sheeran"),
            "beyonce and ed sheeran and jay z"
        );
        assert_eq!(
            normalize_artist_name("Beyoncé & Beyoncé"),
            "beyonce"
        );
        assert_eq!(
            normalize_artist_name("Various Artists & Beyoncé"),
            "beyonce"
        );
        assert_eq!(
            normalize_artist_name("Artist1, Artist2; Artist3 + Artist1"),
            "artist1 and artist2 and artist3"
        );
    }
}

