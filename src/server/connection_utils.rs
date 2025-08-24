use http_body_util::{BodyExt, Empty, Full, combinators::BoxBody};
use reqwest::Url;
use std::{fmt::Debug, path::Path, str, sync::Arc};
use urlencoding::decode;

use hyper::{
    Error as HyperError, Method, Request, Response, Result as HyperResult, StatusCode,
    body::{Body, Bytes},
};

use crate::api::conversion::{ApiClients, convert};

use super::authorization::check_authorization;

pub async fn handle_connection<B: Body + Debug>(
    req: Request<B>,
    api_clients: Arc<ApiClients>,
    api_secret: &str,
) -> HyperResult<Response<BoxBody<Bytes, HyperError>>>
where
    <B as Body>::Error: Debug,
{
    let headers = req.headers();
    let authorization_header = headers.get("Authorization");
    if check_authorization(authorization_header, api_secret).is_err() {
        return Ok(forbidden("Authorization failed"));
    }

    let mut path_it = Path::new(req.uri().path()).iter();
    let _path_root = path_it.next().unwrap().to_str().unwrap();
    let path_resource = match path_it.next() {
        Some(resource) => resource.to_str().unwrap(),
        _none => return Ok(bad_request("Resource cannot be empty")),
    };

    let base = Url::parse("http://localhost").unwrap();
    let full_url = base.join(&req.uri().to_string()).unwrap();

    match (req.method(), path_resource) {
        (&Method::GET, "translate") => {
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
