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
