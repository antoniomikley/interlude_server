use std::collections::HashMap;

use anyhow::bail;
use reqwest::Client;
use serde::Serialize;

use crate::{
    api::ApiClient,
    config::Credentials,
    share_link::{LinkType, ShareLink},
    shared_item::Data,
};

use super::{apple_music::AppleMusicApi, deezer::DeezerApi, spotify::SpotifyApi, tidal::TidalApi};

#[derive(Serialize, Debug, Clone)]
pub struct ConversionResults {
    results: Vec<Link>,
}

#[derive(Serialize, Debug, Clone)]
struct Link {
    provider: String,
    r#type: String,
    #[serde(rename = "displayName")]
    display_name: String,
    url: String,
    artwork: String,
}

impl Link {
    pub fn empty_for(provider: LinkType) -> Self {
        Self {
            provider: provider.to_string(),
            r#type: String::new(),
            display_name: String::new(),
            url: String::new(),
            artwork: String::new(),
        }
    }

    pub fn from_link_and_data(link: &ShareLink, data: &Data) -> Self {
        Self {
            provider: link.link_type.to_string(),
            r#type: data.get_type(),
            display_name: data.get_display_name(),
            url: link.to_url(),
            artwork: String::from(""),
        }
    }
}

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
            tidal = Some(TidalApi::new(client, &credentials.tidal.unwrap()).await);
        }

        Self {
            spotify,
            tidal,
            deezer: None,
            apple_music: None,
        }
    }

    pub fn get_supported_clients(self) -> HashMap<String, ApiClient> {
        let mut supported_apis: HashMap<String, ApiClient> = HashMap::new();

        if self.spotify.is_some() {
            supported_apis.insert(
                LinkType::Spotify.to_string(),
                ApiClient::Spotify(self.spotify.unwrap()),
            );
        }
        if self.tidal.is_some() {
            supported_apis.insert(
                LinkType::Tidal.to_string(),
                ApiClient::Tidal(self.tidal.unwrap()),
            );
        }
        if self.deezer.is_some() {
            supported_apis.insert(
                LinkType::Deezer.to_string(),
                ApiClient::Deezer(self.deezer.unwrap()),
            );
        }
        if self.apple_music.is_some() {
            supported_apis.insert(
                LinkType::AppleMusic.to_string(),
                ApiClient::AppleMusic(self.apple_music.unwrap()),
            );
        }
        return supported_apis;
    }
}

pub async fn convert(url: String, api_clients: ApiClients) -> anyhow::Result<String> {
    let share_link = ShareLink::from_url(&url)?;
    let supported_apis = api_clients.get_supported_clients();

    if !supported_apis.contains_key(&share_link.link_type.to_string()) {
        bail!(
            "Cannot convert links from {}.",
            &share_link.link_type.to_string()
        )
    }

    let data = supported_apis
        .get(&share_link.link_type.to_string())
        .unwrap()
        .link_to_data(&share_link)
        .await?;

    let spotify_result = match supported_apis.get(&LinkType::Spotify.to_string()) {
        Some(client) => {
            let spotify_link = client.data_to_link(&data, &share_link.country_code).await?;
            let spotify_data = client.link_to_data(&spotify_link).await?;
            Link::from_link_and_data(&spotify_link, &spotify_data)
        }
        None => Link::empty_for(LinkType::Spotify),
    };

    let tidal_result = match supported_apis.get(&LinkType::Tidal.to_string()) {
        Some(client) => {
            let tidal_link = client.data_to_link(&data, &share_link.country_code).await?;
            let tidal_data = client.link_to_data(&tidal_link).await?;
            Link::from_link_and_data(&tidal_link, &tidal_data)
        }
        None => Link::empty_for(LinkType::Tidal),
    };

    let deezer_result = match supported_apis.get(&LinkType::Deezer.to_string()) {
        Some(client) => {
            let deezer_link = client.data_to_link(&data, &share_link.country_code).await?;
            let deezer_data = client.link_to_data(&deezer_link).await?;
            Link::from_link_and_data(&deezer_link, &deezer_data)
        }
        None => Link::empty_for(LinkType::Deezer),
    };

    let apple_music_result = match supported_apis.get(&LinkType::AppleMusic.to_string()) {
        Some(client) => {
            let apple_music_link = client.data_to_link(&data, &share_link.country_code).await?;
            let apple_music_data = client.link_to_data(&apple_music_link).await?;
            Link::from_link_and_data(&apple_music_link, &apple_music_data)
        }
        None => Link::empty_for(LinkType::AppleMusic),
    };

    Ok(serde_json::to_string(
        &ConversionResults {
            results: vec![
                spotify_result,
                tidal_result,
                deezer_result,
                apple_music_result,
            ],
        })?
    )
}
