use super::{SongData, artist::ArtistData, norm::normalize_album_title};

#[derive(Clone, Debug)]
pub struct AlbumData {
    pub display_name: String,
    norm_name: String,
    songs: Vec<SongData>,
    artists: Vec<ArtistData>,
    pub upc: String,
}

impl AlbumData {
    pub fn new(name: &str, upc: &str, songs: Vec<SongData>, artists: Vec<ArtistData>) -> Self {
        Self {
            display_name: name.to_owned(),
            norm_name: normalize_album_title(name),
            songs,
            artists,
            upc: upc.to_owned(),
        }
    }
    pub fn with_limited_info(name: &str, upc: &str) -> Self {
        Self {
            display_name: name.to_owned(),
            norm_name: normalize_album_title(name),
            songs: Vec::new(),
            artists: Vec::new(),
            upc: upc.to_owned(),
        }
    }
}

impl PartialEq for AlbumData {
    fn eq(&self, other: &Self) -> bool {
        // if we have the upc we can use it to determine if two albums are the
        // same with relatively high confidence
        if self.upc.len() > 0 {
            if self.upc == other.upc {
                return true;
            }
            return false;
        }

        // if the normalized names do not match, the albums are probably not the same
        if self.norm_name != other.norm_name {
            return false;
        }

        // about 90% of the songs of an album have to be the same, so we can consider two ablbums
        // to also be the same
        let req_song_match_count =
            (std::cmp::max(self.songs.len(), other.songs.len()) as f64 * 0.9).ceil() as usize;
        let mut song_match_count = 0;

        for song in &self.songs {
            if other.songs.contains(song) {
                song_match_count += 1;
            }
        }

        if song_match_count < req_song_match_count {
            return false;
        }

        // as long as one artist of an album is the same
        for artist in &self.artists {
            if other.artists.contains(artist) {
                return true;
            }
        }

        return false;
    }
}
