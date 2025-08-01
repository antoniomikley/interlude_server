use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use dotenv::dotenv;
use hyper::header::HeaderValue;

pub fn get_authorization_token() -> String {
    let _ = dotenv().expect("Failed to load .env file");
    match std::env::var("AUTHORIZATION_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            eprintln!("Authorization token not found in environment variables.");
            std::process::exit(1);
        }
    }
}

pub fn check_authorization(authorization_header: Option<&HeaderValue>) -> Result<(), ()> {
    if let Some(header) = authorization_header {
        if let Ok(header_str) = header.to_str() {
            if header_str.starts_with("Basic ") {
                let b64_part = &header_str[6..];
                match STANDARD.decode(b64_part) {
                    Ok(decoded_bytes) => match str::from_utf8(&decoded_bytes) {
                        Ok(decoded_str) => {
                            if decoded_str == get_authorization_token() {
                                return Ok(());
                            }
                        }
                        Err(_) => {
                            return Err(());
                        }
                    },
                    Err(_) => {
                        return Err(());
                    }
                }
            } else {
                return Err(());
            }
            return Err(());
        }
        return Err(());
    }
    Err(())
}
