use std::{sync::Arc, time::Duration};

use anyhow::bail;
use reqwest::Client;
use rust_iso3166::CountryCode;
use serde::Deserialize;
use tokio::sync::RwLock;

use crate::{
    config::ClientCredentials,
    share_link::{LinkType, ShareLink, ShareObject},
    shared_item::{AlbumData, ArtistData, SongData},
};

use super::{authorization::AccessToken, common::authorized_get_request};
#[derive(Deserialize, Debug, Clone)]
enum ExternalId {
    #[serde(rename = "isrc")]
    ISRC(String),
    #[serde(rename = "upc")]
    UPC(String),
}

#[derive(Deserialize, Debug, Clone)]
struct SongQuery {
    name: String,
    duration_ms: u64,
    external_ids: ExternalId,
    artists: Vec<Artist>,
}
#[derive(Deserialize, Debug, Clone)]
struct Artist {
    name: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Item {
    id: String,
}

#[derive(Clone)]
pub struct SpotifyApi {
    client: Client,
    creds: ClientCredentials,
    access_token: Arc<RwLock<AccessToken>>,
}

impl SpotifyApi {
    const BASE_URL: &'static str = "https://api.spotify.com/v1";
    const AUTH_ENDPOINT: &'static str = "https://accounts.spotify.com/api/token";

    pub async fn new(client: &Client, credentials: &ClientCredentials) -> Self {
        Self {
            client: client.clone(),
            creds: credentials.clone(),
            access_token: Arc::new(RwLock::new(
                AccessToken::new(client, credentials, Self::AUTH_ENDPOINT)
                    .await
                    .unwrap(),
            )),
        }
    }

    async fn get_bearer_token(&self) -> anyhow::Result<String> {
        {
            let token_ro = self.access_token.read().await;
            if !token_ro.is_expired() {
                return Ok(token_ro.token.clone());
            }
        }
        let mut token_w = self.access_token.write().await;
        token_w.refresh(&self.client, &self.creds).await.unwrap();
        return Ok(token_w.token.clone());
    }
    pub async fn get_song_data(&self, song_link: &ShareLink) -> anyhow::Result<SongData> {
        let response = authorized_get_request(
            &self.client,
            format!(
                "{}/tracks/{}?market={}",
                Self::BASE_URL,
                &song_link.id,
                &song_link.country_code.alpha2
            ),
            &self.get_bearer_token().await?,
        )
        .await?;

        let result: SongQuery = serde_json::from_str(&response).unwrap();

        let song_dur = Duration::from_millis(result.duration_ms).as_secs();
        let song_isrc = match result.external_ids {
            ExternalId::ISRC(isrc) => isrc,
            _ => bail!("Should probably return a 503."),
        };

        let mut artists = Vec::new();
        for artist in result.artists {
            artists.push(ArtistData::new(&artist.name, Vec::new()));
        }

        Ok(SongData::new(
            &result.name,
            &song_isrc,
            song_dur,
            Vec::new(),
            artists,
        ))
    }

    pub async fn get_album_data(&self, album_link: &ShareLink) -> anyhow::Result<AlbumData> {
        #[derive(Deserialize, Clone, Debug)]
        struct AlbumQuery {
            name: String,
            external_ids: ExternalId,
        }

        let response = authorized_get_request(
            &self.client,
            format!(
                "{}/albums/{}?market={}",
                Self::BASE_URL,
                &album_link.id,
                &album_link.country_code.alpha2
            ),
            &self.get_bearer_token().await?,
        )
        .await?;

        let result: AlbumQuery = serde_json::from_str(&response).unwrap();

        let upc = match result.external_ids {
            ExternalId::UPC(upc) => upc,
            _ => bail!("also return 503"),
        };

        Ok(AlbumData::with_limited_info(&result.name, &upc, 0))
    }

    pub async fn get_artist_data(&self, _artist_link: ShareLink) -> anyhow::Result<ArtistData> {
        todo!()
    }

    pub async fn get_album_link(
        &self,
        album_data: &AlbumData,
        country_code: &CountryCode,
    ) -> anyhow::Result<ShareLink> {
        #[derive(Deserialize, Debug, Clone)]
        struct AlbumSearch {
            albums: Album,
        }
        #[derive(Deserialize, Debug, Clone)]
        struct Album {
            items: Vec<Item>,
        }
        let response = authorized_get_request(
            &self.client,
            format!(
                "{}/search?q=upc:{}&type=album",
                Self::BASE_URL,
                album_data.upc
            )
            .as_str(),
            &self.get_bearer_token().await?,
        )
        .await?;

        let result: AlbumSearch = serde_json::from_str(&response).unwrap();
        if result.albums.items.len() != 1 {
            bail!("Found no match or found multiple matches. Both is bad.")
        }

        Ok(ShareLink::new(
            LinkType::Spotify,
            ShareObject::Album,
            &result.albums.items[0].id,
            &country_code,
        ))
    }

    pub async fn get_song_link(
        &self,
        song_data: &SongData,
        country_code: &CountryCode,
    ) -> anyhow::Result<ShareLink> {
        #[derive(Deserialize, Debug, Clone)]
        struct TrackSearch {
            tracks: Track,
        }
        #[derive(Deserialize, Debug, Clone)]
        struct Track {
            items: Vec<Item>,
        }

        let response = authorized_get_request(
            &self.client,
            format!(
                "{}/search?q=isrc:{}&type=track",
                Self::BASE_URL,
                song_data.isrc
            ),
            &self.get_bearer_token().await?,
        )
        .await?;

        let result: TrackSearch = serde_json::from_str(&response).unwrap();
        if result.tracks.items.len() == 0 {
            bail!("Found no match.")
        }

        Ok(ShareLink::new(
            LinkType::Spotify,
            ShareObject::Song,
            &result.tracks.items[0].id,
            &country_code,
        ))
    }
}
