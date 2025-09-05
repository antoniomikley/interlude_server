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

pub fn get_providers() -> Vec<Provider> {
    vec![
        Provider {
            name: "Spotify".to_string(),
            url: "https://spotify.com".to_string(),
            logo_url: "https://interlude.api.leshift.de/public/spotify_logo.png".to_string(),
            icon_url: "https://interlude.api.leshift.de/public/spotify_icon.png".to_string(),
        },
        Provider {
            name: "Tidal".to_string(),
            url: "https://tidal.com".to_string(),
            logo_url: "https://interlude.api.leshift.de/public/tidal_logo.png".to_string(),
            icon_url: "https://interlude.api.leshift.de/public/tidal_icon.png".to_string(),
        },
    ]
}
