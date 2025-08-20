use super::{AlbumData, norm::normalize_artist_name};

#[derive(Debug, Clone)]
pub struct ArtistData {
    pub display_name: String,
    norm_name: String,
    albums: Vec<AlbumData>,
}

impl ArtistData {
    pub fn new(name: &str, albums: Vec<AlbumData>) -> Self {
        Self {
            display_name: name.to_owned(),
            norm_name: normalize_artist_name(name),
            albums,
        }
    }

    pub fn without_albums(name: &str) -> Self {
        Self {
            display_name: String::from(name),
            norm_name: normalize_artist_name(name),
            albums: Vec::new(),
        }
    }

    pub fn add_album(&mut self, album: &AlbumData) {
        self.albums.push(album.clone())
    }

    pub fn add_multiple_albums(&mut self, albums: &Vec<AlbumData>) {
        self.albums.extend_from_slice(albums);
    }
}

impl PartialEq for ArtistData {
    fn eq(&self, other: &Self) -> bool {
        // in case an artist has no albums, we have to compare by name
        if self.albums.len() == 0 || other.albums.len() == 0 {
            // if the display name or the normalized name matches, return true
            if self.norm_name != other.norm_name {
                return false;
            }
            return true;
        }
        let req_album_match_count =
            (std::cmp::min(self.albums.len(), other.albums.len()) as f64 * 0.5).ceil() as usize;
        let mut album_match_count = 0;

        for album in &self.albums {
            if other.albums.contains(&album) {
                album_match_count += 1;
                if album_match_count >= req_album_match_count {
                    return true;
                }
            }
        }

        return false;
    }
}
