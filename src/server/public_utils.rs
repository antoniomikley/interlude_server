use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Provider {
    name: String,
    url: String,
    #[serde(rename = "iconUrl")]
    icon_url: String,
}

pub fn get_providers() -> Vec<Provider> {
    vec![
        Provider {
            name: "Spotify".to_string(),
            url: "https://spotify.com".to_string(),
            icon_url: "https://interlude.api.leshift.de/public/spotify.png".to_string(),
        },
        Provider {
            name: "Tidal".to_string(),
            url: "https://tidal.com".to_string(),
            icon_url: "https://interlude.api.leshift.de/public/tidal.png".to_string(),
        },
    ]
}
