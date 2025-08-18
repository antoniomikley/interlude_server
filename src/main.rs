<<<<<<< HEAD
use hyper::{server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use interlude::connection_utils::handle_connection;
use std::net::SocketAddr;

const PORT: u16 = 5000;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind TCP listener");

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let io = TokioIo::new(stream);

        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handle_connection))
                .await
            {
                eprintln!("{}", err);
            }
        });
    }
=======
use interlude::{api::{spotify::SpotifyApi, tidal::TidalApi}, config::Config, share_link::ShareLink};
use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let config = Config::read();
    let creds = config.credentials.unwrap();
    let spotify_creds = creds.spotify.unwrap();
    let tidal_creds= creds.tidal.unwrap();
    let spotify_api = SpotifyApi::new(&client, &spotify_creds).await;
    let tidal_api = TidalApi::new(&client, &tidal_creds).await;

    let song_url = ShareLink::from_url("https://tidal.com/browse/track/338872652?u").unwrap();
    let album_data = tidal_api.get_song_data(song_url.clone()).await.unwrap();
    println!("{:?}", spotify_api.get_song_link(album_data, song_url.country_code).await.unwrap().to_url())
>>>>>>> 44548c1 (implement conversion between spotify and tidal for tracks and albums)
}
