use reqwest::Client;
use rust_iso3166::CountryCode;
use serde::Deserialize;

use crate::{
    share_link::{LinkType, ShareLink, ShareObject},
    shared_item::{AlbumData, ArtistData, SongData},
};

use super::ApiError;

#[derive(Clone)]
pub struct DeezerApi {
    client: Client,
}

impl DeezerApi {
    const BASE_URL: &'static str = "https://api.deezer.com";

    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub async fn get_song_data(&self, song_link: &ShareLink) -> Result<SongData, ApiError> {
        #[derive(Deserialize)]
        struct SongQuery {
            title: String,
            isrc: String,
            duration: u64,
            album: AlbumInfo,
            artist: ArtistInfo,
        }
        #[derive(Deserialize)]
        struct AlbumInfo {
            id: u64,
        }
        #[derive(Deserialize)]
        struct ArtistInfo {
            name: String,
        }

        let response = self
            .client
            .get(format!("{}/track/{}", Self::BASE_URL, song_link.id))
            .send()
            .await?
            .text()
            .await?;

        let song_info: SongQuery = serde_json::from_str(&response)?;
        let album_link = ShareLink::new(
            LinkType::Deezer,
            ShareObject::Album,
            &song_info.album.id.to_string(),
            &song_link.country_code,
        );

        let album_data = self.get_album_data(&album_link).await?;
        let artist_data = ArtistData::new(&song_info.artist.name, vec![album_data.clone()]);

        Ok(SongData::new(
            &song_info.title,
            &song_info.isrc,
            song_info.duration,
            vec![album_data],
            vec![artist_data],
        ))
    }

    pub async fn get_album_data(&self, album_link: &ShareLink) -> Result<AlbumData, ApiError> {
        #[derive(Deserialize, Debug)]
        struct AlbumQuery {
            title: String,
            upc: String,
        }

        let response = self
            .client
            .get(format!("{}/album/{}", Self::BASE_URL, album_link.id))
            .send()
            .await?
            .text()
            .await?;
        let album_info: AlbumQuery = serde_json::from_str(&response)?;

        Ok(AlbumData::with_limited_info(
            &album_info.title,
            &album_info.upc,
        ))
    }

    pub async fn get_artist_data(&self, _artist_link: ShareLink) -> Result<ArtistData, ApiError> {
        todo!()
    }

    pub async fn get_song_link(
        &self,
        song_data: &SongData,
        country_code: &CountryCode,
    ) -> Result<ShareLink, ApiError> {
        #[derive(Deserialize)]
        struct SongQuery {
            id: u64,
        }

        let response = self
            .client
            .get(format!("{}/track/isrc:{}", Self::BASE_URL, song_data.isrc))
            .send()
            .await?
            .text()
            .await?;
        let song_info: SongQuery = serde_json::from_str(&response)?;
        Ok(ShareLink::new(
            LinkType::Deezer,
            ShareObject::Song,
            &song_info.id.to_string(),
            country_code,
        ))
    }

    pub async fn get_album_link(
        &self,
        album_data: &AlbumData,
        country_code: &CountryCode,
    ) -> Result<ShareLink, ApiError> {
        #[derive(Deserialize)]
        struct AlbumQuery {
            id: u64,
        }

        let response = self
            .client
            .get(format!("{}/album/upc:{}", Self::BASE_URL, album_data.upc))
            .send()
            .await?
            .text()
            .await?;
        let album_info: AlbumQuery = serde_json::from_str(&response)?;
        Ok(ShareLink::new(
            LinkType::Deezer,
            ShareObject::Album,
            &album_info.id.to_string(),
            country_code,
        ))
    }

    pub async fn get_cover_art(&self, album_data: &AlbumData) -> Result<String, ApiError> {
        #[derive(Deserialize)]
        struct AlbumQuery {
            cover: String,
            cover_small: Option<String>,
            cover_medium: Option<String>,
            cover_big: Option<String>,
        }
        let response = self
            .client
            .get(format!("{}/album/upc:{}", Self::BASE_URL, album_data.upc))
            .send()
            .await?
            .text()
            .await?;

        let album_info: AlbumQuery = serde_json::from_str(&response)?;

        if album_info.cover_medium.is_some() {
            return Ok(album_info.cover_medium.unwrap());
        }
        if album_info.cover_small.is_some() {
            return Ok(album_info.cover_small.unwrap());
        }
        if album_info.cover_big.is_some() {
            return Ok(album_info.cover_big.unwrap());
        }
        return Ok(album_info.cover);
    }
}
