use std::fmt;

pub enum LinkType {
    Tidal,
    Spotify,
    AppleMusic,
    Any,
}

impl fmt::Display for LinkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinkType::Tidal => write!(f, "Tidal"),
            LinkType::Spotify => write!(f, "Spotify"),
            LinkType::AppleMusic => write!(f, "Apple Music"),
            LinkType::Any => write!(f, "Any"),
        }
    }
}
