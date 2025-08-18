use anyhow::bail;
use rust_iso3166::CountryCode;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum LinkType {
    Spotify,
    Tidal,
    AppleMusic,
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
            },
            LinkType::Spotify => {
                return format!("https://open.spotify.com/{}/{}", obj, self.id);
            },
            LinkType::AppleMusic => {
                return format!("https://music.apple.com/{}/{}/{}", self.country_code.alpha2.to_ascii_lowercase(), obj, self.id)
            }
        }
    }

    pub fn new(link_type: LinkType, share_obj: ShareObject, id: &str, country_code: CountryCode) -> Self {
        Self {
            link_type,
            country_code,
            share_obj,
            id: id.to_owned()
        }
    }

    pub fn from_url(url: &str) -> anyhow::Result<ShareLink> {
        const ERR_MSG_INVALID_URL: &str = "The provided URL is not valid.";
        const ERR_MSG_INVALID_SHARE_LINK: &str = "The provided link is not a valid share link.";

        let mut country_code: Option<CountryCode> = None;
        let mut id = String::new();

        let mut parts = url.split('/');
        match parts.next() {
            Some("https:") => {},
            Some(_) => { 
                bail!("This is not a valid share link, since it does not use HTTPS.")
            },
            None => bail!(ERR_MSG_INVALID_URL)
        }

        match parts.next() { 
            Some("") => {},
            Some(_) | None => bail!(ERR_MSG_INVALID_URL),
        }

        let link_type: Option<LinkType> = match parts.next() {
            None => bail!(ERR_MSG_INVALID_URL),
            Some(text) => {
                match text {
                    "open.spotify.com" => Some(LinkType::Spotify),
                    "tidal.com" => Some(LinkType::Tidal),
                    "music.apple.com" => Some(LinkType::AppleMusic),
                    _ => bail!("The provided link is not supported.")
                }
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
                                },
                                _ => bail!("The provided link is not a valid Spotify share link. The country code is malformed."),
                            }
                        } else {
                            country_code = match rust_iso3166::from_alpha2(&text[5..].to_ascii_uppercase()) {
                                Some(cc) => Some(cc),
                                None => bail!("The provided link does not contain a valid iso3166-1 alpha-2 country code."),
                            };
                        }
                    },
                    None => bail!("The provided link is not a valid Spotify share link."),
                }
            },
            Some(LinkType::Tidal) => {
                match parts.next() {
                    Some("browse") => country_code = Some(rust_iso3166::from_alpha2("US").unwrap()),
                    _ => bail!("The provided link is not a valid Tidal share link.")
                }
            },
            Some(LinkType::AppleMusic) => {
                match parts.next() {
                    Some(cc) => {
                        let cc = rust_iso3166::from_alpha2(&cc.to_ascii_uppercase());
                        if cc.is_none() {bail!("The provided link does not contain a valid iso3166-1 alpha-2 country code.")}
                        country_code = Some(cc.unwrap());
                    },
                    _ => {},
                }
            }
            None => {}, // Cannot be None at this point. Function failes already before.
        }

        let share_obj: Option<ShareObject> = match parts.next() {
            Some("track") => Some(ShareObject::Song),
            Some("album") => Some(ShareObject::Album),
            Some("artist") => Some(ShareObject::Artist),
            Some("song") => {
                if link_type != Some(LinkType::AppleMusic) {
                    bail!(ERR_MSG_INVALID_SHARE_LINK)
                }
                Some(ShareObject::Song)
            }
            Some(_) | None => bail!(ERR_MSG_INVALID_SHARE_LINK)
        };

        // In case of Apple Music we have to consume the next part of the url, which usually
        // contains the name of the song, album or artist, which we don't need at this point.
        if link_type == Some(LinkType::AppleMusic) {
            if parts.next().is_none() {
                bail!(ERR_MSG_INVALID_SHARE_LINK)
            }
        }

        match parts.next() {
            Some(text) => {
                if parts.next().is_some() {
                    bail!(ERR_MSG_INVALID_SHARE_LINK);
                }
                for c in text.chars() {
                    match c {
                        '?' => {
                            if id.len() == 0 {
                                bail!(ERR_MSG_INVALID_SHARE_LINK);
                            }
                            break;
                        }
                        _ => id.push(c)
                    }
                }
            }
            None => bail!(ERR_MSG_INVALID_SHARE_LINK),
        }
        Ok(ShareLink{
            link_type: link_type.unwrap(),
            country_code: country_code.unwrap(),
            share_obj: share_obj.unwrap(),
            id
        })
    }
}
