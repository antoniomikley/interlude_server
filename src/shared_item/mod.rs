pub mod song;
pub mod album;
pub mod artist;
pub mod norm;

pub use song::SongData;
pub use album::AlbumData;
pub use artist::ArtistData;

pub enum Data {
    Song(SongData),
    Album(AlbumData),
    Artist(ArtistData),
}

impl Data {
    pub fn get_type(&self) -> String {
        match self {
            Self::Song(_) => String::from("Song"),
            Self::Album(_) => String::from("Album"),
            Self::Artist(_) => String::from("Artist"),
        }
    }
    pub fn get_display_name(&self) -> String {
        match self {
            Self::Song(data) => data.display_name.clone(),
            Self::Album(data) => data.display_name.clone(),
            Self::Artist(data) => data.display_name.clone()
        }
    }
}
