use anyhow::bail;
use apple_music::AppleMusicApi;
use deezer::DeezerApi;
use rust_iso3166::CountryCode;
use spotify::SpotifyApi;
use tidal::TidalApi;

use crate::{share_link::{ShareLink, ShareObject}, shared_item::Data};

pub mod tidal;
pub mod spotify;
pub mod apple_music;
pub mod deezer;
pub mod authorization;
pub mod conversion;
pub mod common;

pub enum ApiClient {
    Spotify(SpotifyApi),
    Tidal(TidalApi),
    Deezer(DeezerApi),
    AppleMusic(AppleMusicApi)
}

impl ApiClient {
    pub async fn link_to_data(&self, link: &ShareLink) -> anyhow::Result<Data> {
        match link.share_obj {
            ShareObject::Song => {
                match self {
                    ApiClient::Spotify(client) => return Ok(Data::Song(client.get_song_data(&link).await?)),
                    ApiClient::Tidal(client) => return Ok(Data::Song(client.get_song_data(&link).await?)),
                    ApiClient::Deezer(_) => bail!("Deezer is currently not supported."),
                    ApiClient::AppleMusic(_) => bail!("AppleMusic is currently not supported.")
                }
            },
            ShareObject::Album => {
                match self {
                    ApiClient::Spotify(client) => return Ok(Data::Album(client.get_album_data(&link).await?)),
                    ApiClient::Tidal(client) => return Ok(Data::Album(client.get_album_data(&link).await?)),
                    ApiClient::Deezer(_) => bail!("Deezer is currently not supported."),
                    ApiClient::AppleMusic(_) => bail!("AppleMusic is currently not supported.")
                }
            }
            ShareObject::Artist => bail!("Conversion for artist links is currently not supported.")
        }
    }
    pub async fn data_to_link(&self, data: &Data, country_code: &CountryCode) -> anyhow::Result<ShareLink> {
        match data {
            Data::Song(song_data) => {
                match self {
                    ApiClient::Spotify(client) => return client.get_song_link(&song_data, country_code).await,
                    ApiClient::Tidal(client) => return client.get_song_link(&song_data, country_code).await,
                    ApiClient::Deezer(_) => bail!("Deezer is currently not supported."),
                    ApiClient::AppleMusic(_) => bail!("AppleMusic is currently not supported.")
                }
            },
            Data::Album(album_data) => {
                match self {
                    ApiClient::Spotify(client) => return client.get_album_link(&album_data, country_code).await,
                    ApiClient::Tidal(client) => return client.get_album_link(&album_data, country_code).await,
                    ApiClient::Deezer(_) => bail!("Deezer is currently not supported."),
                    ApiClient::AppleMusic(_) => bail!("AppleMusic is currently not supported.")
                }
            },
            Data::Artist(_artist_data) => bail!("Conversion for artist links is currently not supported.")
        }
    }
}
