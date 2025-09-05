use apple_music::AppleMusicApi;
use authorization::AuthorizationError;
use deezer::DeezerApi;
use rust_iso3166::CountryCode;
use spotify::SpotifyApi;
use thiserror::Error;
use tidal::TidalApi;

use crate::{
    share_link::{ShareLink, ShareObject},
    shared_item::Data,
};

pub mod apple_music;
pub mod authorization;
pub mod conversion;
pub mod deezer;
pub mod spotify;
pub mod tidal;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error(transparent)]
    ParsingError(#[from] serde_json::Error),
    #[error("The provided link does not have the correct attributes.")]
    UnsuitableLink,
    #[error(transparent)]
    AuthorizationError(#[from] AuthorizationError),
    #[error("Found incorrect attributes in the response.")]
    IncorrectAttributes,
    #[error("Conversion unsuccessful.")]
    UnsuccessfulConversion,
    #[error("This feature is currently not supported")]
    UnsupportedFeature,
}

pub enum ApiClient {
    Spotify(SpotifyApi),
    Tidal(TidalApi),
    Deezer(DeezerApi),
    AppleMusic(AppleMusicApi),
}

impl ApiClient {
    pub async fn link_to_data(&self, link: &ShareLink) -> Result<Data, ApiError> {
        match link.share_obj {
            ShareObject::Song => match self {
                ApiClient::Spotify(client) => Ok(Data::Song(client.get_song_data(&link).await?)),
                ApiClient::Tidal(client) => Ok(Data::Song(client.get_song_data(&link).await?)),
                ApiClient::Deezer(client) => Ok(Data::Song(client.get_song_data(&link).await?)),
                ApiClient::AppleMusic(_) => Err(ApiError::UnsupportedFeature),
            },
            ShareObject::Album => match self {
                ApiClient::Spotify(client) => Ok(Data::Album(client.get_album_data(&link).await?)),
                ApiClient::Tidal(client) => Ok(Data::Album(client.get_album_data(&link).await?)),
                ApiClient::Deezer(client) => Ok(Data::Album(client.get_album_data(&link).await?)),
                ApiClient::AppleMusic(_) => Err(ApiError::UnsupportedFeature),
            },
            ShareObject::Artist => Err(ApiError::UnsupportedFeature),
        }
    }
    pub async fn data_to_link(
        &self,
        data: &Data,
        country_code: &CountryCode,
    ) -> Result<ShareLink, ApiError> {
        match data {
            Data::Song(song_data) => match self {
                ApiClient::Spotify(client) => client.get_song_link(&song_data, country_code).await,
                ApiClient::Tidal(client) => client.get_song_link(&song_data, country_code).await,
                ApiClient::Deezer(client) => client.get_song_link(&song_data, country_code).await,
                ApiClient::AppleMusic(_) => Err(ApiError::UnsupportedFeature),
            },
            Data::Album(album_data) => match self {
                ApiClient::Spotify(client) => {
                    client.get_album_link(&album_data, country_code).await
                }
                ApiClient::Tidal(client) => client.get_album_link(&album_data, country_code).await,
                ApiClient::Deezer(client) => client.get_album_link(&album_data, country_code).await,
                ApiClient::AppleMusic(_) => Err(ApiError::UnsupportedFeature),
            },
            Data::Artist(_artist_data) => Err(ApiError::UnsupportedFeature),
        }
    }

    pub async fn get_artwork(
        &self,
        data: &Data,
        country_code: &CountryCode,
    ) -> Result<String, ApiError> {
        match self {
            ApiClient::Tidal(client) => match data {
                Data::Song(song_data) => {
                    client
                        .get_cover_art(&song_data.albums[0], country_code)
                        .await
                }
                Data::Album(album_data) => client.get_cover_art(&album_data, country_code).await,
                Data::Artist(_) => Err(ApiError::UnsupportedFeature),
            },
            ApiClient::Spotify(client) => match data {
                Data::Song(song_data) => client.get_cover_art(&song_data.albums[0]).await,
                Data::Album(album_data) => client.get_cover_art(&album_data).await,
                Data::Artist(_) => Err(ApiError::UnsupportedFeature),
            },
            ApiClient::Deezer(client) => match data {
                Data::Song(song_data) => client.get_cover_art(&song_data.albums[0]).await,
                Data::Album(album_data) => client.get_cover_art(&album_data).await,
                Data::Artist(_) => Err(ApiError::UnsupportedFeature),
            },
            ApiClient::AppleMusic(_) => Err(ApiError::UnsupportedFeature),
        }
    }
}
