use serde::{Serialize, Deserialize};

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
    pub deezer: Option<ClientCredentials>,
}

impl Credentials {
    pub fn empty() -> Self {
        Self {
            tidal: None,
            spotify: None,
            apple_music: None,
            deezer: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub credentials: Option<Credentials>,
}


impl Config {
    pub fn read() -> Self {
        let config_file_contents = match std::fs::read("./Config.toml") {
            Ok(contents) => contents,
            Err(_) => {
                std::fs::write("./Config.toml", toml::to_string_pretty(&Self::default()).unwrap().as_bytes()).expect("Could not write Config.toml.");
                std::fs::read("./Config.toml").expect("Can not read Config.toml.")
            }
        };

        toml::from_slice(&config_file_contents).expect("Config.toml is malformed.")
    }

    pub fn default() -> Self {
        Self {
            credentials: Some(Credentials::empty()),
        }
    }
}
