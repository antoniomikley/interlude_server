use hyper::{Method, Request, StatusCode, body::Bytes};

#[tokio::test]
async fn test_handle_connection_success() {
    let req = Request::builder()
        .method(Method::GET)
        .uri("/translate/some_link/any")
        .header("Authorization", "Basic dXNlcjpwYXNzd29yZA==") // user:password in base64
        .body(Bytes::new())
        .unwrap();

    let response = handle_connection(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_handle_connection_error() {
    let req = Request::builder()
        .method(Method::GET)
        .uri("/translate/some_link/any")
        .header("Authorization", "Basic 1989zr1ipufbpv") // user:password in base64
        .body(full(Bytes::new()))
        .unwrap();

    let response = handle_connection(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}
