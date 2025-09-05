use std::sync::Arc;

use interlude::{
    api::conversion::{convert, ApiClients, ConversionResults, Link},
    config::Config,
};
use once_cell::sync::Lazy;
use reqwest::Client;

static CONFIG: &'static str = include_str!("../Config.toml");
static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::new()
});

#[tokio::test]
async fn convert_spotify_song_link() {
    let config: Config = toml::from_str(CONFIG).unwrap();
    let client = CLIENT.clone();
    let api_clients = Arc::new(
        ApiClients::new(&client, config.clone().credentials.expect("No credentials found.")).await,
    );

    let url = "https://open.spotify.com/track/2HBBM75Xv3o2Mqdyh1NcM0?si=fb796f70fcb6449c";

    let conversion: ConversionResults = serde_json::from_str(&convert(url, api_clients).await.unwrap()).unwrap();
    let spotify_result = Link {
      provider: "Spotify".to_string(),
      r#type: "Song".to_string(),
      display_name: "Heavy Is the Crown".to_string(),
      url: "https://open.spotify.com/track/5Aw7tCjLgKTAF1mRXQfVHm".to_string(),
      artwork: "https://i.scdn.co/image/ab67616d00001e02b11a5489e8cb11dd22b930a0".to_string()
    }; 
    let tidal_result = Link {
      provider: "Tidal".to_string(),
      r#type: "Song".to_string(),
      display_name: "Heavy Is the Crown".to_string(),
      url: "https://tidal.com/browse/track/387265136".to_string(),
      artwork: "https://resources.tidal.com/images/3f49a481/68e5/46e4/a57a/5da8a75aa106/320x320.jpg".to_string()
    };
    let deezer_result = Link {
      provider: "Deezer".to_string(),
      r#type: "Song".to_string(),
      display_name: "Heavy Is the Crown".to_string(),
      url: "https://www.deezer.com/track/2994098971".to_string(),
      artwork: "https://cdn-images.dzcdn.net/images/cover/1e8ffbd401303b5693226c12ee0b84fb/250x250-000000-80-0-0.jpg".to_string()
    };

    assert_eq!(conversion.results.contains(&spotify_result), true);
    assert_eq!(conversion.results.contains(&tidal_result), true);
    assert_eq!(conversion.results.contains(&deezer_result), true);
}

#[tokio::test]
async fn convert_spotify_album_link() {
    let config: Config = toml::from_str(CONFIG).unwrap();
    let client = CLIENT.clone();
    let api_clients = Arc::new(
        ApiClients::new(&client, config.clone().credentials.expect("No credentials found.")).await,
    );

    println!("{:?}", &config.clone());
    let url = "https://open.spotify.com/album/4OXoBlapQygTdzAifJm8BL?si=bDeSpXKjTV2sKYMNej33Pw";

    let conversion: ConversionResults = serde_json::from_str(&convert(url, api_clients).await.unwrap()).unwrap();
    let spotify_result = Link {
      provider: "Spotify".to_string(),
      r#type: "Album".to_string(),
      display_name: "Eternal Blue".to_string(),
      url: "https://open.spotify.com/album/4OXoBlapQygTdzAifJm8BL".to_string(),
      artwork: "https://i.scdn.co/image/ab67616d00001e023e234c82f96fa4ded8e5ca47".to_string()
    }; 
    let tidal_result = Link  {
      provider: "Tidal".to_string(),
      r#type: "Album".to_string(),
      display_name: "Eternal Blue".to_string(),
      url: "https://tidal.com/browse/album/194372122".to_string(),
      artwork: "https://resources.tidal.com/images/22968d83/ae3b/47fc/90db/a94fcd0036df/320x320.jpg".to_string()
    };
    let deezer_result = Link {
      provider: "Deezer".to_string(),
      r#type: "Album".to_string(),
      display_name: "Eternal Blue".to_string(),
      url: "https://www.deezer.com/album/252187122".to_string(),
      artwork: "https://cdn-images.dzcdn.net/images/cover/e858dbae6f773cdb34f0c7fa47a526d8/250x250-000000-80-0-0.jpg".to_string()
    };

    assert_eq!(conversion.results.contains(&spotify_result), true);
    assert_eq!(conversion.results.contains(&tidal_result), true);
    assert_eq!(conversion.results.contains(&deezer_result), true);
}

#[tokio::test]
async fn convert_tidal_song_link() {
    let config: Config = toml::from_str(CONFIG).unwrap();
    let client = CLIENT.clone();
    let api_clients = Arc::new(
        ApiClients::new(&client, config.credentials.expect("No credentials found.")).await,
    );

    let url = "https://tidal.com/browse/track/1885625/u";

    let conversion: ConversionResults = serde_json::from_str(&convert(url, api_clients).await.unwrap()).unwrap();
    let spotify_result = Link {
      provider: "Spotify".to_string(),
      r#type: "Song".to_string(),
      display_name: "Snuff".to_string(),
      url: "https://open.spotify.com/track/0p6ZIbYw39oaAQX93tpETN".to_string(),
      artwork: "https://i.scdn.co/image/ab67616d00001e02457163bec7e8e4decf8c6375".to_string()
    }; 
    let tidal_result = Link  {
      provider: "Tidal".to_string(),
      r#type: "Song".to_string(),
      display_name: "Snuff".to_string(),
      url: "https://tidal.com/browse/track/1885625".to_string(),
      artwork: "https://resources.tidal.com/images/4f837fa6/edbc/41c0/8a7f/656cb3ddf004/320x320.jpg".to_string()
    };
    let deezer_result = Link {
      provider: "Deezer".to_string(),
      r#type: "Song".to_string(),
      display_name: "Snuff".to_string(),
      url: "https://www.deezer.com/track/1195567".to_string(),
      artwork: "https://cdn-images.dzcdn.net/images/cover/3d4d0fe601be67cb2e13654d40d7101a/250x250-000000-80-0-0.jpg".to_string()
    };

    assert_eq!(conversion.results.contains(&spotify_result), true);
    assert_eq!(conversion.results.contains(&tidal_result), true);
    assert_eq!(conversion.results.contains(&deezer_result), true);
}

#[tokio::test]
async fn convert_tidal_album_link() {
    let config: Config = toml::from_str(CONFIG).unwrap();
    let client = CLIENT.clone();
    let api_clients = Arc::new(
        ApiClients::new(&client, config.credentials.expect("No credentials found.")).await,
    );

    let url = "https://tidal.com/browse/album/1885614/u";

    let conversion: ConversionResults = serde_json::from_str(&convert(url, api_clients).await.unwrap()).unwrap();
    let spotify_result = Link    {
      provider: "Spotify".to_string(),
      r#type: "Album".to_string(),
      display_name: "All Hope Is Gone".to_string(),
      url: "https://open.spotify.com/album/0hFWapnP7orzXCMwNU5DuA".to_string(),
      artwork: "https://i.scdn.co/image/ab67616d00001e02457163bec7e8e4decf8c6375".to_string()
    }; 
    let tidal_result = Link  {
      provider: "Tidal".to_string(),
      r#type: "Album".to_string(),
      display_name: "All Hope Is Gone".to_string(),
      url: "https://tidal.com/browse/album/1885614".to_string(),
      artwork: "https://resources.tidal.com/images/4f837fa6/edbc/41c0/8a7f/656cb3ddf004/320x320.jpg".to_string()
    };
    let deezer_result = Link  {
      provider: "Deezer".to_string(),
      r#type: "Album".to_string(),
      display_name: "All Hope Is Gone".to_string(),
      url: "https://www.deezer.com/album/127402".to_string(),
      artwork: "https://cdn-images.dzcdn.net/images/cover/3d4d0fe601be67cb2e13654d40d7101a/250x250-000000-80-0-0.jpg".to_string()
    };

    assert_eq!(conversion.results.contains(&spotify_result), true);
    assert_eq!(conversion.results.contains(&tidal_result), true);
    assert_eq!(conversion.results.contains(&deezer_result), true);
}

#[tokio::test]
async fn convert_deezer_song_link() {
    let config: Config = toml::from_str(CONFIG).unwrap();
    let client = CLIENT.clone();
    let api_clients = Arc::new(
        ApiClients::new(&client, config.credentials.expect("No credentials found.")).await,
    );

    let url = "https://link.deezer.com/s/30X12yMuBSBgGoX01n05M";

    let conversion: ConversionResults = serde_json::from_str(&convert(url, api_clients).await.unwrap()).unwrap();
    let spotify_result = Link     {
      provider: "Spotify".to_string(),
      r#type: "Song".to_string(),
      display_name: "Look To Windward".to_string(),
      url: "https://open.spotify.com/track/4Lojbtk7XNMdSKRHSFbdkm".to_string(),
      artwork: "https://i.scdn.co/image/ab67616d00001e020e48dcb579fd8e59d0a3c218".to_string()
    }; 
    let tidal_result = Link  {
      provider: "Tidal".to_string(),
      r#type: "Song".to_string(),
      display_name: "Look To Windward".to_string(),
      url: "https://tidal.com/browse/track/434030392".to_string(),
      artwork: "https://resources.tidal.com/images/5e7d37ee/9c40/4388/9e51/4d49e67b4310/320x320.jpg".to_string()
    };
    let deezer_result = Link  {
      provider: "Deezer".to_string(),
      r#type: "Song".to_string(),
      display_name: "Look To Windward".to_string(),
      url: "https://www.deezer.com/track/3330723931".to_string(),
      artwork: "https://cdn-images.dzcdn.net/images/cover/88cb1ee2758133c9dd4514deea199c0b/250x250-000000-80-0-0.jpg".to_string()
    };

    assert_eq!(conversion.results.contains(&spotify_result), true);
    assert_eq!(conversion.results.contains(&tidal_result), true);
    assert_eq!(conversion.results.contains(&deezer_result), true);
}

#[tokio::test]
async fn convert_deezer_album_link() {
    let config: Config = toml::from_str(CONFIG).unwrap();
    let client = CLIENT.clone();
    let api_clients = Arc::new(
        ApiClients::new(&client, config.credentials.expect("No credentials found.")).await,
    );

    let url = "https://link.deezer.com/s/30X1h4uTFd7R8JUxoPuJV";

    let conversion: ConversionResults = serde_json::from_str(&convert(url, api_clients).await.unwrap()).unwrap();
    let spotify_result = Link     {
      provider: "Spotify".to_string(),
      r#type: "Album".to_string(),
      display_name: "Even In Arcadia".to_string(),
      url: "https://open.spotify.com/album/1lS7FeRcSUuIGqyg99UGpj".to_string(),
      artwork: "https://i.scdn.co/image/ab67616d00001e020e48dcb579fd8e59d0a3c218".to_string()
    }; 
    let tidal_result = Link   {
      provider: "Tidal".to_string(),
      r#type: "Album".to_string(),
      display_name: "Even In Arcadia".to_string(),
      url: "https://tidal.com/browse/album/434030391".to_string(),
      artwork: "https://resources.tidal.com/images/5e7d37ee/9c40/4388/9e51/4d49e67b4310/320x320.jpg".to_string()
    };
    let deezer_result = Link   {
      provider: "Deezer".to_string(),
      r#type: "Album".to_string(),
      display_name: "Even In Arcadia".to_string(),
      url: "https://www.deezer.com/album/744506781".to_string(),
      artwork: "https://cdn-images.dzcdn.net/images/cover/88cb1ee2758133c9dd4514deea199c0b/250x250-000000-80-0-0.jpg".to_string()
    };

    assert_eq!(conversion.results.contains(&spotify_result), true);
    assert_eq!(conversion.results.contains(&tidal_result), true);
    assert_eq!(conversion.results.contains(&deezer_result), true);
}
