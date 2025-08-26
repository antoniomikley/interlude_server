use rust_iso3166::CountryCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShareLinkError {
    #[error("The provided URL is invalid.")]
    InvalidUrl,
    #[error("The provided URL is not an accepted share link.")]
    NotAShareLink,
    #[error("The provided share link is invalid or malformed.")]
    MalformedOrInvalidLink,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum LinkType {
    Spotify,
    Tidal,
    AppleMusic,
    Deezer,
}

impl LinkType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Tidal => String::from("Tidal"),
            Self::Spotify => String::from("Spotify"),
            Self::Deezer => String::from("Deezer"),
            Self::AppleMusic => String::from("AppleMusic"),
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum ShareObject {
    Song,
    Album,
    Artist,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShareLink {
    pub link_type: LinkType,
    pub country_code: CountryCode,
    pub share_obj: ShareObject,
    pub id: String,
}

impl ShareLink {
    pub fn to_url(&self) -> String {
        let obj = match self.share_obj {
            ShareObject::Album => "album",
            ShareObject::Song => "track",
            ShareObject::Artist => "artist",
        };
        match self.link_type {
            LinkType::Tidal => {
                return format!("https://tidal.com/browse/{}/{}", obj, self.id);
            }
            LinkType::Spotify => {
                return format!("https://open.spotify.com/{}/{}", obj, self.id);
            }
            LinkType::AppleMusic => {
                todo!()
            }
            LinkType::Deezer => {
                todo!()
            }
        }
    }

    pub fn new(
        link_type: LinkType,
        share_obj: ShareObject,
        id: &str,
        country_code: &CountryCode,
    ) -> Self {
        Self {
            link_type,
            country_code: country_code.clone(),
            share_obj,
            id: id.to_owned(),
        }
    }

    pub fn from_url(url: &str) -> Result<ShareLink, ShareLinkError> {
        let mut country_code: Option<CountryCode> = None;
        let mut id = String::new();

        let mut parts = url.split('/');
        match parts.next() {
            Some("https:") => {}
            Some(_) => return Err(ShareLinkError::InvalidUrl),
            None => return Err(ShareLinkError::InvalidUrl),
        }

        match parts.next() {
            Some("") => {}
            Some(_) | None => return Err(ShareLinkError::InvalidUrl),
        }

        let link_type: Option<LinkType> = match parts.next() {
            None => return Err(ShareLinkError::InvalidUrl),
            Some(text) => match text {
                "open.spotify.com" => Some(LinkType::Spotify),
                "tidal.com" => Some(LinkType::Tidal),
                "music.apple.com" => Some(LinkType::AppleMusic),
                _ => return Err(ShareLinkError::NotAShareLink),
            },
        };

        match link_type {
            Some(LinkType::Spotify) => {
                let parts_backup = parts.clone();
                match parts.next() {
                    Some(text) => {
                        if text.len() != 7 || !text.starts_with("intl-") {
                            match text {
                                "track" | "album" | "artist" => {
                                    parts = parts_backup;
                                    country_code = Some(rust_iso3166::from_alpha2("US").unwrap());
                                }
                                _ => return Err(ShareLinkError::MalformedOrInvalidLink),
                            }
                        } else {
                            country_code =
                                match rust_iso3166::from_alpha2(&text[5..].to_ascii_uppercase()) {
                                    Some(cc) => Some(cc),
                                    None => return Err(ShareLinkError::MalformedOrInvalidLink),
                                };
                        }
                    }
                    None => return Err(ShareLinkError::MalformedOrInvalidLink),
                }
            }
            Some(LinkType::Tidal) => match parts.next() {
                Some("browse") => country_code = Some(rust_iso3166::from_alpha2("US").unwrap()),
                _ => return Err(ShareLinkError::MalformedOrInvalidLink),
            },
            Some(LinkType::AppleMusic) => match parts.next() {
                Some(cc) => {
                    let cc = rust_iso3166::from_alpha2(&cc.to_ascii_uppercase());
                    if cc.is_none() {
                        return Err(ShareLinkError::MalformedOrInvalidLink);
                    }
                    country_code = Some(cc.unwrap());
                }
                _ => {}
            },
            Some(LinkType::Deezer) => {
                todo!();
            }
            None => {} // Cannot be None at this point. Function failes already before.
        }

        let share_obj: Option<ShareObject> = match parts.next() {
            Some("track") => Some(ShareObject::Song),
            Some("album") => Some(ShareObject::Album),
            Some("artist") => Some(ShareObject::Artist),
            Some("song") => {
                if link_type != Some(LinkType::AppleMusic) {
                    return Err(ShareLinkError::MalformedOrInvalidLink);
                }
                Some(ShareObject::Song)
            }
            Some(_) | None => return Err(ShareLinkError::MalformedOrInvalidLink),
        };

        // In case of Apple Music we have to consume the next part of the url, which usually
        // contains the name of the song, album or artist, which we don't need at this point.
        if link_type == Some(LinkType::AppleMusic) {
            if parts.next().is_none() {
                return Err(ShareLinkError::MalformedOrInvalidLink);
            }
        }

        match parts.next() {
            Some(text) => {
                if parts.next().is_some() {
                    return Err(ShareLinkError::MalformedOrInvalidLink);
                }
                for c in text.chars() {
                    match c {
                        '?' => {
                            if id.len() == 0 {
                                return Err(ShareLinkError::MalformedOrInvalidLink);
                            }
                            break;
                        }
                        _ => id.push(c),
                    }
                }
            }
            None => return Err(ShareLinkError::MalformedOrInvalidLink),
        }
        Ok(ShareLink {
            link_type: link_type.unwrap(),
            country_code: country_code.unwrap(),
            share_obj: share_obj.unwrap(),
            id,
        })
    }
}
