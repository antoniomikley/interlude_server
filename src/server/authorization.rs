use core::str;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use hyper::header::HeaderValue;


pub fn check_authorization(authorization_header: Option<&HeaderValue>, api_secret: &str) -> Result<(), ()> {
    if let Some(header) = authorization_header {
        if let Ok(header_str) = header.to_str() {
            if header_str.starts_with("Bearer ") {
                let b64_part = &header_str[7..];
                match STANDARD.decode(b64_part) {
                    Ok(decoded_bytes) => match str::from_utf8(&decoded_bytes) {
                        Ok(decoded_str) => {
                            if decoded_str == api_secret {
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
