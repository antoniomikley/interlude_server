use http_body_util::{BodyExt, Empty, Full, combinators::BoxBody};
use std::{fmt::Debug, path::Path, str};

use hyper::{
    Error as HyperError, Method, Request, Response, Result as HyperResult, StatusCode,
    body::{Body, Bytes},
};

use crate::authorization::check_authorization;

pub async fn handle_connection<B: Body + Debug>(
    req: Request<B>,
) -> HyperResult<Response<BoxBody<Bytes, HyperError>>>
where
    <B as Body>::Error: Debug,
{
    let headers = req.headers();
    let authorization_header = headers.get("Authorization");
    if let Err(_) = check_authorization(authorization_header) {
        return Ok(forbidden("Authorization failed"));
    }

    let mut path_it = Path::new(req.uri().path()).iter();
    let _path_root = path_it.next().unwrap().to_str().unwrap();
    let path_resource = match path_it.next() {
        Some(resource) => resource.to_str().unwrap(),
        _none => return Ok(bad_request("Resource cannot be empty")),
    };

    match (req.method(), path_resource) {
        (&Method::GET, "translate") => {
            let link = match path_it.next() {
                Some(link) => link.to_str().unwrap(),
                _none => return Ok(bad_request("Link must be provided")),
            };

            let translate_type = match path_it.next() {
                Some(translate_type) => translate_type.to_str().unwrap(),
                _none => "any",
            };

            let return_link = format!("Translated link for: {}, type: {}", link, translate_type);

            let body = full(Bytes::from(return_link.to_string()));
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(body)
                .unwrap();

            Ok(response)
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
