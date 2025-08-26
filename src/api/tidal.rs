use std::sync::Arc;

use tokio::sync::RwLock;

use anyhow::bail;
use reqwest::Client;
use rust_iso3166::CountryCode;
use serde::Deserialize;

use crate::{
    config::ClientCredentials,
    share_link::{LinkType, ShareLink, ShareObject},
    shared_item::{AlbumData, ArtistData, SongData},
};

use super::{
    ApiError,
    authorization::{AccessToken, AuthorizationError},
};

#[derive(Deserialize, Debug, Clone)]
struct QueryResult {
    data: Data,
    included: Option<Vec<Data>>,
}

#[derive(Debug, Clone, Deserialize)]
struct FilterQuery {
    data: Vec<Data>,
}

#[derive(Deserialize, Debug, Clone)]
struct Data {
    id: String,
    #[serde(flatten)]
    attributes: Attributes,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "attributes")]
enum Attributes {
    #[serde(rename = "tracks")]
    Tracks(TrackAttrs),
    #[serde(rename = "albums")]
    Albums(AlbumAttrs),
    #[serde(rename = "artists")]
    Artists(ArtistAttrs),
}

#[derive(Deserialize, Debug, Clone)]
struct TrackAttrs {
    pub title: String,
    pub isrc: String,
    pub duration: String,
}

#[derive(Deserialize, Debug, Clone)]
struct AlbumAttrs {
    pub title: String,
    #[serde(alias = "barcodeId")]
    pub upc: String,
    pub duration: String,
}

#[derive(Deserialize, Debug, Clone)]
struct ArtistAttrs {
    pub name: String,
}

#[derive(Clone)]
pub struct TidalApi {
    client: Client,
    creds: ClientCredentials,
    access_token: Arc<RwLock<AccessToken>>,
}

const ISO_DURATION_ERR_MSG: &'static str = "Duration does not follow iso8601.";
const INCLUDE_ERR_MSG: &'static str =
    "Request with include query parameter did not return included in response.";

impl TidalApi {
    const BASE_URL: &'static str = "https://openapi.tidal.com/v2";
    const AUTH_ENDPOINT: &'static str = "https://auth.tidal.com/v1/oauth2/token";

    pub async fn new(
        client: &Client,
        credentials: &ClientCredentials,
    ) -> Result<Self, AuthorizationError> {
        Ok(Self {
            client: client.clone(),
            creds: credentials.clone(),
            access_token: Arc::new(RwLock::new(
                AccessToken::new(client, credentials, Self::AUTH_ENDPOINT).await?,
            )),
        })
    }

    async fn get_bearer_token(&self) -> Result<String, AuthorizationError> {
        {
            let token_ro = self.access_token.read().await;
            if !token_ro.is_expired() {
                return Ok(token_ro.token.clone());
            }
        }
        let mut token_w = self.access_token.write().await;
        token_w.refresh(&self.client, &self.creds).await?;
        return Ok(token_w.token.clone());
    }

    pub async fn get_song_data(&self, song_link: &ShareLink) -> Result<SongData, ApiError> {
        if song_link.link_type != LinkType::Tidal || song_link.share_obj != ShareObject::Song {
            return Err(ApiError::UnsuitableLink);
        }

        let cc = song_link.country_code.clone();
        let id = song_link.id.clone();

        let response = self
            .client
            .get(format!(
                "{}/tracks/{}?countryCode={}&include=albums,artists",
                Self::BASE_URL,
                id,
                cc.alpha2
            ))
            .bearer_auth(self.get_bearer_token().await?)
            .send()
            .await?
            .text()
            .await?;

        let results: QueryResult = serde_json::from_str(&response)?;

        let song_attrs = match results.data.attributes {
            Attributes::Tracks(attrs) => attrs,
            _ => return Err(ApiError::IncorrectAttributes),
        };
        let song_name = song_attrs.title;
        let song_isrc = song_attrs.isrc;
        let song_dur = iso8601_to_seconds(&song_attrs.duration).unwrap();

        let mut albums = Vec::new();
        let mut artists = Vec::new();

        for include in results.included.expect(INCLUDE_ERR_MSG) {
            match include.attributes {
                Attributes::Tracks(_) => return Err(ApiError::IncorrectAttributes),
                Attributes::Albums(attrs) => albums.push(AlbumData::with_limited_info(
                    &attrs.title,
                    &attrs.upc,
                    iso8601_to_seconds(&attrs.duration).expect(ISO_DURATION_ERR_MSG),
                )),
                Attributes::Artists(attrs) => artists.push(ArtistData::without_albums(&attrs.name)),
            }
        }

        Ok(SongData::new(
            &song_name, &song_isrc, song_dur, albums, artists,
        ))
    }

    pub async fn get_album_data(&self, album_link: &ShareLink) -> Result<AlbumData, ApiError> {
        if album_link.link_type != LinkType::Tidal || album_link.share_obj != ShareObject::Album {
            return Err(ApiError::UnsuitableLink);
        }

        let cc = album_link.country_code.clone();
        let id = album_link.id.clone();

        let response = self
            .client
            .get(format!(
                "{}/albums/{}?countryCode={}&include=items,artists",
                Self::BASE_URL,
                id,
                cc.alpha2
            ))
            .bearer_auth(self.get_bearer_token().await?)
            .send()
            .await?
            .text()
            .await?;

        let results: QueryResult = serde_json::from_str(&response)?;
        let album_attrs = match results.data.attributes {
            Attributes::Albums(attrs) => attrs,
            _ => return Err(ApiError::IncorrectAttributes),
        };

        let album_name = album_attrs.title;
        let album_dur = iso8601_to_seconds(&album_attrs.duration).expect(ISO_DURATION_ERR_MSG);
        let album_upc = album_attrs.upc;
        let mut songs: Vec<SongData> = Vec::new();
        let mut artists: Vec<ArtistData> = Vec::new();
        let includes = results.included.expect(INCLUDE_ERR_MSG);

        for include in includes {
            match include.attributes {
                Attributes::Artists(attrs) => artists.push(ArtistData::without_albums(&attrs.name)),
                Attributes::Tracks(attrs) => songs.push(SongData::new(
                    &attrs.title,
                    &attrs.isrc,
                    iso8601_to_seconds(&attrs.duration).expect(ISO_DURATION_ERR_MSG),
                    Vec::new(),
                    Vec::new(),
                )),
                _ => {}
            }
        }

        Ok(AlbumData::new(
            &album_name,
            &album_upc,
            album_dur,
            songs,
            artists,
        ))
    }

    pub async fn get_artist_data(&self, artist_link: ShareLink) -> Result<ArtistData, ApiError> {
        #[derive(Deserialize, Debug, Clone)]
        struct RelationshipResults {
            links: Link,
            included: Vec<Data>,
        }

        #[derive(Deserialize, Debug, Clone)]
        struct Link {
            next: Option<String>,
        }

        if artist_link.link_type != LinkType::Tidal || artist_link.share_obj != ShareObject::Artist
        {
            return Err(ApiError::UnsuitableLink);
        }

        let cc = artist_link.country_code.clone();
        let id = artist_link.id.clone();

        let response = self
            .client
            .get(format!(
                "{}/artists/{}?countryCode={}",
                Self::BASE_URL,
                id,
                cc.alpha2
            ))
            .bearer_auth(self.get_bearer_token().await?)
            .send()
            .await?
            .text()
            .await?;

        let results: QueryResult = serde_json::from_str(&response)?;
        let artist_attrs = match results.data.attributes {
            Attributes::Artists(attrs) => attrs,
            _ => return Err(ApiError::IncorrectAttributes),
        };

        let artist_name = artist_attrs.name;

        let mut albums: Vec<AlbumData> = Vec::new();
        let response = self
            .client
            .get(format!(
                "{}/artists/{}/relationships/albums?countryCode={}&include=albums",
                Self::BASE_URL,
                id,
                cc.alpha2
            ))
            .bearer_auth(self.get_bearer_token().await?)
            .send()
            .await?
            .text()
            .await?;

        let mut results: RelationshipResults = serde_json::from_str(&response)?;

        // TODO: This needs a better solution
        let max_requests = 3; // in development mode a tidal application can make max 10 requests
        let mut request_counter = 0;

        loop {
            for item in results.included {
                let album_attrs = match item.attributes {
                    Attributes::Albums(attrs) => attrs,
                    _ => return Err(ApiError::IncorrectAttributes),
                };

                albums.push(AlbumData::with_limited_info(
                    &album_attrs.title,
                    &album_attrs.upc,
                    iso8601_to_seconds(&album_attrs.duration).expect(ISO_DURATION_ERR_MSG),
                ))
            }

            if request_counter == max_requests {
                break;
            }

            match results.links.next {
                None => break,
                Some(link) => {
                    let response = self
                        .client
                        .get(format!("{}{}", Self::BASE_URL, &link))
                        .bearer_auth(self.get_bearer_token().await?)
                        .send()
                        .await?
                        .text()
                        .await?;

                    request_counter += 1;
                    results = serde_json::from_str(&response)?;
                }
            }
        }

        return Ok(ArtistData::new(&artist_name, albums));
    }

    pub async fn get_song_link(
        &self,
        song_data: &SongData,
        country_code: &CountryCode,
    ) -> Result<ShareLink, ApiError> {
        let response = self
            .client
            .get(format!(
                "{}/tracks?countryCode={}&filter[isrc]={}",
                Self::BASE_URL,
                country_code.alpha2,
                song_data.isrc
            ))
            .bearer_auth(self.get_bearer_token().await?)
            .send()
            .await?
            .text()
            .await?;

        let results: FilterQuery = serde_json::from_str(&response)?;
        for item in results.data {
            let share_link =
                ShareLink::new(LinkType::Tidal, ShareObject::Song, &item.id, country_code);
            let sd = self.get_song_data(&share_link).await?;
            if sd == song_data.clone() {
                return Ok(share_link);
            }
        }
        Err(ApiError::UnsuccessfulConversion)
    }

    pub async fn get_album_link(
        &self,
        album_data: &AlbumData,
        country_code: &CountryCode,
    ) -> Result<ShareLink, ApiError> {
        let response = self
            .client
            .get(format!(
                "{}/albums?countryCode={}&filter[barcodeId]={}",
                Self::BASE_URL,
                country_code.alpha2,
                album_data.upc
            ))
            .bearer_auth(self.get_bearer_token().await?)
            .send()
            .await?
            .text()
            .await?;

        let results: FilterQuery = serde_json::from_str(&response)?;
        for item in results.data {
            let share_link =
                ShareLink::new(LinkType::Tidal, ShareObject::Album, &item.id, &country_code);
            let ad = self.get_album_data(&share_link).await.unwrap();
            if &ad == album_data {
                return Ok(share_link);
            }
        }

        Err(ApiError::UnsuccessfulConversion)
    }
}

fn iso8601_to_seconds(iso8601_duration: &str) -> anyhow::Result<u64> {
    let iso_dur = iso8601::duration(iso8601_duration).unwrap();
    match iso_dur {
        iso8601::Duration::Weeks(_) => bail!("Week long durations  are not supported."),
        iso8601::Duration::YMDHMS {
            year,
            month,
            day,
            hour,
            minute,
            second,
            millisecond,
        } => {
            if year != 0 {
                bail!("Year long durations are not supported.")
            }
            if month != 0 {
                bail!("Month long durations are not supported.")
            }
            let mut seconds: u64 = if millisecond > 500 { 1 } else { 0 };
            seconds += u64::from(second);
            seconds += u64::from(minute) * 60;
            seconds += u64::from(hour) * 60 * 60;
            seconds += u64::from(day) * 24 * 60 * 60;
            return Ok(seconds);
        }
    }
}
