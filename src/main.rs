use hyper::{server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use interlude::{
    api::conversion::ApiClients, config::Config, server::connection_utils::handle_connection,
};
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    let config = Config::read();
    let client = reqwest::Client::new();
    let api_clients = Arc::new(
        ApiClients::new(&client, config.credentials.expect("No credentials found.")).await,
    );
    let api_secret = config.api_password.expect("api_password_not set.");

    let addr = SocketAddr::from((config.listen_address_ipv4, config.listen_port));
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind TCP listener");

    loop {
        let api_clients = Arc::clone(&api_clients);
        let (stream, _) = listener.accept().await.unwrap();
        let io = TokioIo::new(stream);
        let api_secret = api_secret.clone();

        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(|req| handle_connection(req, api_clients.clone(), &api_secret)),
                )
                .await
            {
                eprintln!("{}", err);
            }
        });
    }
}
