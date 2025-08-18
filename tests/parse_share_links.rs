use interlude::share_link::{LinkType, ShareLink};

#[test]
fn parse_spotify_song_link() {
    let url = "https://open.spotify.com/intl-de/track/36puuD04lEUD8kVwQsTLm6?si=8847fe60c51b48c6";
    let expected_result = ShareLink {
        link_type: LinkType::Spotify,
        country_code: rust_iso3166::from_alpha2("DE").unwrap(),
        share_obj: interlude::share_link::ShareObject::Song,
        id: String::from("36puuD04lEUD8kVwQsTLm6"),
    };
    assert_eq!(expected_result, ShareLink::from_url(url).unwrap());
}

#[test]
fn parse_spotify_album_link() {
    let url =
        "https://open.spotify.com/intl-de/album/1EOHCAqQjeA1hNXsJTlzFF?si=VW3NCFmeQbCbDghECoxOsw";
    let expected_result = ShareLink {
        link_type: LinkType::Spotify,
        country_code: rust_iso3166::from_alpha2("DE").unwrap(),
        share_obj: interlude::share_link::ShareObject::Album,
        id: String::from("1EOHCAqQjeA1hNXsJTlzFF"),
    };
    assert_eq!(expected_result, ShareLink::from_url(url).unwrap());
}

#[test]
fn parse_spotify_artist_link() {
    let url =
        "https://open.spotify.com/intl-de/artist/3IrUyDPQlQFcB5lMWhPml2?si=RwPO_G32QWqfHw0vhpSx0Q";
    let expected_result = ShareLink {
        link_type: LinkType::Spotify,
        country_code: rust_iso3166::from_alpha2("DE").unwrap(),
        share_obj: interlude::share_link::ShareObject::Artist,
        id: String::from("3IrUyDPQlQFcB5lMWhPml2"),
    };
    assert_eq!(expected_result, ShareLink::from_url(url).unwrap());
}

#[test]
fn parse_tidal_song_link() {
    let url = "https://tidal.com/browse/track/300807510?u";
    let expected_result = ShareLink {
        link_type: LinkType::Tidal,
        country_code: rust_iso3166::from_alpha2("US").unwrap(),
        share_obj: interlude::share_link::ShareObject::Song,
        id: String::from("300807510"),
    };
    assert_eq!(expected_result, ShareLink::from_url(url).unwrap());
}

#[test]
fn parse_tidal_album_link() {
    let url = "https://tidal.com/browse/album/412502324?u";
    let expected_result = ShareLink {
        link_type: LinkType::Tidal,
        country_code: rust_iso3166::from_alpha2("US").unwrap(),
        share_obj: interlude::share_link::ShareObject::Album,
        id: String::from("412502324"),
    };
    assert_eq!(expected_result, ShareLink::from_url(url).unwrap());
}

#[test]
fn parse_tidal_artist_link() {
    let url = "https://tidal.com/browse/artist/5036395?u";
    let expected_result = ShareLink {
        link_type: LinkType::Tidal,
        country_code: rust_iso3166::from_alpha2("US").unwrap(),
        share_obj: interlude::share_link::ShareObject::Artist,
        id: String::from("5036395"),
    };
    assert_eq!(expected_result, ShareLink::from_url(url).unwrap());
}

#[test]
fn parse_apple_song_link() {
    let url = "https://music.apple.com/us/song/what-was-that/1810905307";
    let expected_result = ShareLink {
        link_type: LinkType::AppleMusic,
        country_code: rust_iso3166::from_alpha2("US").unwrap(),
        share_obj: interlude::share_link::ShareObject::Song,
        id: String::from("1810905307"),
    };
    assert_eq!(expected_result, ShareLink::from_url(url).unwrap());
}

#[test]
fn parse_apple_album_link() {
    let url = "https://music.apple.com/us/album/virgin/1810905299";
    let expected_result = ShareLink {
        link_type: LinkType::AppleMusic,
        country_code: rust_iso3166::from_alpha2("US").unwrap(),
        share_obj: interlude::share_link::ShareObject::Album,
        id: String::from("1810905299"),
    };
    assert_eq!(expected_result, ShareLink::from_url(url).unwrap());
}

#[test]
fn parse_apple_artist_link() {
    let url = "https://music.apple.com/us/artist/lorde/602767352";
    let expected_result = ShareLink {
        link_type: LinkType::AppleMusic,
        country_code: rust_iso3166::from_alpha2("US").unwrap(),
        share_obj: interlude::share_link::ShareObject::Artist,
        id: String::from("602767352"),
    };
    assert_eq!(expected_result, ShareLink::from_url(url).unwrap());
}
