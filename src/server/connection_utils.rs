use http_body_util::{BodyExt, Empty, Full, combinators::BoxBody};
use mime_guess::from_path;
use reqwest::Url;
use std::{fmt::Debug, path::Path, sync::Arc};
use urlencoding::decode;

use hyper::{
    Error as HyperError, Method, Request, Response, Result as HyperResult, StatusCode,
    body::{Body, Bytes},
};

use crate::{
    api::conversion::{ApiClients, convert},
    server::public_utils::get_providers,
};

use super::authorization::check_authorization;

pub async fn handle_connection<B: Body + Debug>(
    req: Request<B>,
    api_clients: Arc<ApiClients>,
    api_secret: &str,
    ext_addr: &str,
) -> HyperResult<Response<BoxBody<Bytes, HyperError>>>
where
    <B as Body>::Error: Debug,
{
    let mut path_it = Path::new(req.uri().path()).iter();
    let _path_root = path_it.next().unwrap().to_str().unwrap();
    let path_resource = match path_it.next() {
        Some(resource) => resource.to_str().unwrap(),
        _none => return Ok(bad_request("Resource cannot be empty")),
    };

    // Skip authorization for public endpoint
    if path_resource != "public" {
        let headers = req.headers();
        let authorization_header = headers.get("Authorization");
        if check_authorization(authorization_header, api_secret).is_err() {
            return Ok(forbidden("Authorization failed"));
        }
    }

    let base = Url::parse("http://localhost").unwrap();
    let full_url = base.join(&req.uri().to_string()).unwrap();

    match (req.method(), path_resource) {
        (&Method::GET, "convert") => {
            let link = match full_url.query_pairs().find(|(key, _)| key == "link") {
                Some(link) => decode(&link.1).unwrap().to_string(),
                _none => return Ok(bad_request("Link must be provided")),
            };
            let return_link = convert(&link, api_clients).await;
            match return_link {
                Ok(return_link) => {
                    let body = full(Bytes::from(return_link));
                    let response = Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "application/json")
                        .body(body)
                        .unwrap();

                    Ok(response)
                }
                Err(err) => {
                    let body = full(Bytes::from(err.to_string()));
                    let response = Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(body)
                        .unwrap();
                    Ok(response)
                }
            }
        }
        (&Method::GET, "providers") => {
            let providers = serde_json::to_string(&get_providers(ext_addr)).unwrap();
            let body = full(Bytes::from(providers));
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(body)
                .unwrap();

            Ok(response)
        }
        (&Method::GET, "public") => {
            let mut path_it = Path::new(req.uri().path()).iter();
            path_it.next();
            path_it.next();
            let maybe_file = path_it.next();

            if maybe_file.is_none() {
                // falls jemand nur /public aufruft → Bad Request
                return Ok(bad_request("Filename must be provided"));
            }

            let rel = maybe_file.unwrap().to_str().unwrap();
            if rel.contains("..") {
                return Ok(bad_request("Invalid path"));
            }

            let file_path = Path::new("public").join(rel);
            match tokio::fs::read(&file_path).await {
                Ok(contents) => {
                    let mime = from_path(&file_path)
                        .first_or_octet_stream()
                        .essence_str()
                        .to_string();

                    let body = full(Bytes::from(contents));
                    let response = Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", mime)
                        .body(body)
                        .unwrap();
                    Ok(response)
                }
                Err(_err) => {
                    let body = full(Bytes::from("File not found"));
                    let response = Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .header("Content-Type", "text/plain")
                        .body(body)
                        .unwrap();
                    return Ok(response);
                }
            }
        }
        _ => Ok(bad_request("Invalid method or resource")),
    }
}

pub fn empty() -> BoxBody<Bytes, HyperError> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

pub fn full<T: Into<Bytes>>(data: T) -> BoxBody<Bytes, HyperError> {
    Full::new(data.into())
        .map_err(|never| match never {})
        .boxed()
}

pub fn bad_request(message: &str) -> Response<BoxBody<Bytes, HyperError>> {
    let body: BoxBody<Bytes, HyperError> = full(Bytes::from(message.to_string()));
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header("Content-Type", "text/plain")
        .body(body)
        .unwrap()
}

pub fn forbidden(message: &str) -> Response<BoxBody<Bytes, HyperError>> {
    let body: BoxBody<Bytes, HyperError> = full(Bytes::from(message.to_string()));
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .header("Content-Type", "text/plain")
        .body(body)
        .unwrap()
}
