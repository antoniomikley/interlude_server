use super::{album::AlbumData, artist::ArtistData, norm::normalize_song_title};

#[derive(Debug, Clone)]
pub struct SongData {
    pub display_name: String,
    norm_name: String,
    pub isrc: String,
    duration: u64,
    pub albums: Vec<AlbumData>,
    artists: Vec<ArtistData>,
}

impl SongData {
    pub fn new(
        name: &str,
        isrc: &str,
        duration: u64,
        albums: Vec<AlbumData>,
        artists: Vec<ArtistData>,
    ) -> Self {
        Self {
            display_name: name.to_owned(),
            norm_name: normalize_song_title(name),
            isrc: isrc.to_owned(),
            duration,
            albums,
            artists,
        }
    }
}

impl PartialEq for SongData {
    fn eq(&self, other: &Self) -> bool {
        let mut name_match = true;
        let mut dur_match = true;
        let mut isrc_match = self.isrc.len() != 0;
        let mut album_match = false;
        let mut artist_match = false;

        if self.isrc != other.isrc {
            isrc_match = false;
        }

        if self.norm_name != other.norm_name {
            name_match = false;
        }

        if !(self.duration >= other.duration - 2 && self.duration <= other.duration + 2) {
            dur_match = false;
        }

        for album in &self.albums {
            if other.albums.contains(album) {
                album_match = true;
                break;
            }
        }

        for artist in &self.artists {
            if other.artists.contains(artist) {
                artist_match = true;
                break;
            }
        }

        let mdata_match = name_match && album_match && artist_match;

        // if ISRCs match then the songs are probably the same, if duration also matches we can be
        // pretty confident
        if isrc_match && dur_match {
            return true;
        }

        // even if duration does not match, if the rest of the metadata is the same than we can
        // also be pretty confident
        if isrc_match && mdata_match {
            return true;
        }

        // even if ISRC does not match, if duration and metadata match then its pretty much
        // a toss up but we still return true
        // if dur_match && mdata_match {
        //     return true;
        // }

        return false;
    }
}
