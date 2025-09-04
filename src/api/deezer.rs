use reqwest::Client;
use rust_iso3166::CountryCode;
use serde::Deserialize;

use crate::{share_link::ShareLink, shared_item::{AlbumData, ArtistData, SongData}};

use super::ApiError;

#[derive(Clone)]
pub struct DeezerApi {
    client: Client,
}

impl DeezerApi {
    const BASE_URL: &'static str = "https://api.deezer.com/";

    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone()
        }
    }

    pub async fn get_song_data(&self, song_link: &ShareLink) -> Result<SongData, ApiError> {
        #[derive(Deserialize)]
        struct SongQuery {

        }
        todo!()
    }

    pub async fn get_album_data(&self, album_link: &ShareLink) -> Result<AlbumData, ApiError> {
        todo!()
    }

    pub async fn get_artist_data(&self, artist_link: ShareLink) -> Result<ArtistData, ApiError> {
        todo!()
    }


    pub async fn get_song_link(
        &self,
        song_data: &SongData,
        country_code: &CountryCode,
    ) -> Result<ShareLink, ApiError> {
        todo!()
    }

    pub async fn get_album_link(
        &self,
        album_data: &AlbumData,
        country_code: &CountryCode,
    ) -> Result<ShareLink, ApiError> {
        todo!()
    }
    const PREFERRED_MAX_IMAGE_SIZE: u16 = 800;
    const PREFERRED_MIN_IMAGE_SIZE: u16 = 300;

    pub async fn get_cover_art(
        &self,
        album_data: &AlbumData,
        country_code: &CountryCode,
    ) -> Result<String, ApiError> {
        todo!()
    }
}
