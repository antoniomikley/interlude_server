use std::{collections::HashMap, sync::Arc};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    api::ApiClient,
    config::Credentials,
    share_link::{LinkType, ShareLink, ShareLinkError},
    shared_item::Data,
};

use super::{
    ApiError, apple_music::AppleMusicApi, deezer::DeezerApi, spotify::SpotifyApi, tidal::TidalApi,
};

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error(transparent)]
    ApiClient(#[from] ApiError),
    #[error(transparent)]
    Link(#[from] ShareLinkError),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConversionResults {
    pub results: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Link {
    pub provider: String,
    pub r#type: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub url: String,
    pub artwork: String,
}

impl Link {
    pub fn new(link: &ShareLink, data: &Data, artwork: &str) -> Self {
        Self {
            provider: link.link_type.to_string(),
            r#type: data.get_type(),
            display_name: data.get_display_name(),
            url: link.to_url(),
            artwork: artwork.to_owned(),
        }
    }
}

#[derive(Clone)]
pub struct ApiClients {
    spotify: Option<SpotifyApi>,
    tidal: Option<TidalApi>,
    deezer: Option<DeezerApi>,
    apple_music: Option<AppleMusicApi>,
}

impl ApiClients {
    pub async fn new(client: &Client, credentials: Credentials) -> Self {
        let mut spotify = None;
        let mut tidal = None;

        if credentials.spotify.is_some() {
            spotify = Some(SpotifyApi::new(client, &credentials.spotify.unwrap()).await);
        }
        if credentials.tidal.is_some() {
            tidal = Some(
                TidalApi::new(client, &credentials.tidal.unwrap())
                    .await
                    .unwrap(),
            );
        }

        Self {
            spotify,
            tidal,
            deezer: Some(DeezerApi::new(client)),
            apple_music: None,
        }
    }

    pub fn get_supported_clients(&self) -> HashMap<String, ApiClient> {
        let mut supported_apis: HashMap<String, ApiClient> = HashMap::new();

        let cloned = self.clone();
        if self.spotify.is_some() {
            supported_apis.insert(
                LinkType::Spotify.to_string(),
                ApiClient::Spotify(cloned.spotify.unwrap()),
            );
        }
        if self.tidal.is_some() {
            supported_apis.insert(
                LinkType::Tidal.to_string(),
                ApiClient::Tidal(cloned.tidal.unwrap()),
            );
        }
        if self.deezer.is_some() {
            supported_apis.insert(
                LinkType::Deezer.to_string(),
                ApiClient::Deezer(cloned.deezer.unwrap()),
            );
        }
        if self.apple_music.is_some() {
            supported_apis.insert(
                LinkType::AppleMusic.to_string(),
                ApiClient::AppleMusic(cloned.apple_music.unwrap()),
            );
        }
        return supported_apis;
    }
}

pub async fn convert(url: &str, api_clients: Arc<ApiClients>) -> Result<String, ConversionError> {
    let share_link = ShareLink::from_url(&url).await?;
    let supported_apis = api_clients.get_supported_clients();

    if !supported_apis.contains_key(&share_link.link_type.to_string()) {
        return Err(ConversionError::ApiClient(ApiError::UnsupportedFeature));
    }

    let data = supported_apis
        .get(&share_link.link_type.to_string())
        .unwrap()
        .link_to_data(&share_link)
        .await?;

    let spotify_result = match supported_apis.get(&LinkType::Spotify.to_string()) {
        Some(client) => {
            let spotify_link = client.data_to_link(&data, &share_link.country_code).await;
            let image_link = client
                .get_artwork(&data, &share_link.country_code)
                .await
                .unwrap_or(String::new());
            if spotify_link.is_ok() {
                let spotify_link = spotify_link.unwrap();
                let spotify_data = client.link_to_data(&spotify_link).await.unwrap();
                Some(Link::new(&spotify_link, &spotify_data, &image_link))
            } else {
                None
            }
        }
        None => None,
    };

    let tidal_result = match supported_apis.get(&LinkType::Tidal.to_string()) {
        Some(client) => {
            let tidal_link = client.data_to_link(&data, &share_link.country_code).await;
            let image_link = client
                .get_artwork(&data, &share_link.country_code)
                .await
                .unwrap_or(String::new());
            if tidal_link.is_ok() {
                let tidal_link = tidal_link.unwrap();
                let tidal_data = client.link_to_data(&tidal_link).await?;
                Some(Link::new(&tidal_link, &tidal_data, &image_link))
            } else {
                None
            }
        }
        None => None,
    };

    let deezer_result = match supported_apis.get(&LinkType::Deezer.to_string()) {
        Some(client) => {
            let deezer_link = client.data_to_link(&data, &share_link.country_code).await;
            let image_link = client
                .get_artwork(&data, &share_link.country_code)
                .await
                .unwrap_or(String::new());

            if deezer_link.is_ok() {
                let deezer_link = deezer_link.unwrap();
                let deezer_data = client.link_to_data(&deezer_link).await.unwrap();
                Some(Link::new(&deezer_link, &deezer_data, &image_link))
            } else {
                None
            }
        }
        None => None,
    };

    let apple_music_result = match supported_apis.get(&LinkType::AppleMusic.to_string()) {
        Some(client) => {
            let apple_music_link = client.data_to_link(&data, &share_link.country_code).await?;
            let apple_music_data = client.link_to_data(&apple_music_link).await?;
            Some(Link::new(&apple_music_link, &apple_music_data, ""))
        }
        None => None,
    };

    let mut results: Vec<Link> = Vec::new();

    if spotify_result.is_some() {
        results.push(spotify_result.unwrap());
    }
    if tidal_result.is_some() {
        results.push(tidal_result.unwrap());
    }
    if deezer_result.is_some() {
        results.push(deezer_result.unwrap());
    }
    if apple_music_result.is_some() {
        results.push(apple_music_result.unwrap());
    }

    Ok(serde_json::to_string(&ConversionResults { results })
        .expect("Conversion result should always be valid."))
}
