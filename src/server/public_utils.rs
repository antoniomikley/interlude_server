use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Provider {
    name: String,
    url: String,
    #[serde(rename = "logoUrl")]
    logo_url: String,
    #[serde(rename = "iconUrl")]
    icon_url: String,
}

pub fn get_providers(ext_addr: &str) -> Vec<Provider> {
    vec![
        Provider {
            name: "Spotify".to_string(),
            url: "https://spotify.com".to_string(),
            logo_url: format!("{}/public/spotify_logo.png", ext_addr),
            icon_url: format!("{}/public/spotify_icon.png", ext_addr),
        },
        Provider {
            name: "Tidal".to_string(),
            url: "https://tidal.com".to_string(),
            logo_url: format!("{}/public/tidal_logo.png", ext_addr),
            icon_url: format!("{}/public/tidal_icon.png", ext_addr),
        },
        Provider {
            name: "Deezer".to_string(),
            url: "https://www.deezer.com".to_string(),
            logo_url: format!("{}/public/deezer_logo.png", ext_addr),
            icon_url: format!("{}/public/deezer_icon.png", ext_addr),
        },
    ]
}
