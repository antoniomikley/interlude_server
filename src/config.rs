use std::{net::Ipv4Addr, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCredentials {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub tidal: Option<ClientCredentials>,
    pub spotify: Option<ClientCredentials>,
    pub apple_music: Option<AccessToken>,
}

impl Credentials {
    pub fn empty() -> Self {
        Self {
            tidal: None,
            spotify: None,
            apple_music: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub credentials: Option<Credentials>,
    pub api_password: Option<String>,
    pub listen_address_ipv4: Ipv4Addr,
    pub listen_port: u16,
}

impl Config {
    pub fn read() -> Self {
        let config_file_contents = match std::fs::read("./Config.toml") {
            Ok(contents) => contents,
            Err(_) => {
                std::fs::write(
                    "./Config.toml",
                    toml::to_string_pretty(&Self::default()).unwrap().as_bytes(),
                )
                .expect("Could not write Config.toml.");
                std::fs::read("./Config.toml").expect("Can not read Config.toml.")
            }
        };

        toml::from_slice(&config_file_contents).expect("Config.toml is malformed.")
    }

    pub fn default() -> Self {
        Self {
            credentials: Some(Credentials::empty()),
            api_password: Some(String::from("")),
            listen_address_ipv4: Ipv4Addr::from_str("0.0.0.0").unwrap(),
            listen_port: 5000,
        }
    }
}
