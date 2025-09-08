# Interlude Server
Convert share links of one music streaming service to another. Currently supported are:

|           | Spotify   | Tidal     | Deezer    | Apple Music   |
|-----------|-----------|-----------|-----------|---------------|
|Songs      | ✅        | ✅        | ✅        | ❌            |
|Albums     | ✅        | ✅        | ✅        | ❌            |
|Artists    | ❌        | ❌        | ❌        | ❌            |

There are several clients that make use of the Interlude server that offer a comfortable user experience:
- [Interlude Android App](https://github.com/LS-Studios/Interlude-Mobile-Client)
- [Interlude Web Client](https://github.com/LS-Studios/Interlude-Web-Client)
---
- [Installation](#installation)
    - [Cargo](#cargo)
    - [GitHub Release](#github-release)
    - [Building from Source](#building-from-source)
    - [Docker](#docker)
- [Configuration](#configuration)
- [Usage](#usage)
    - [Link Conversion](#link-conversion)
    - [Query supported streaming services](#query-supported-streaming-services)
    - [Access public files](#access-public-files)

---
## Installation 
### Cargo
The easiest way to install interlude_server is using `cargo`.
If you have the Rust toolchain setup, the latest version can be installed using the following command:
```bash
cargo install --locked interlude
```
### GitHub Release
An alternative way to run the interlude server is using one of the provided release executables.
There are Windows and Linux releases for x86_64, although the Windows version is not signed so Windows might complain.

### Building from source
To build the application from source you need to have the rust toolchain installed. 
Clone this repo and run `cargo build --release` to compile the application. 
The binary can be found under `./target/release/interlude`.
```bash
git clone https://github.com/antoniomikley/interlude_server
cd interlude_server
cargo build --release
```

### Docker
The server can also be run using Docker. A Docker Compose file is provided.
```bash
# Docker Compose verwenden (Port 30002)
docker-compose up -d

# Oder manuell:
docker build -t interlude-server .
docker run -p 30002:5000 -v ./Config.toml:/app/Config.toml:ro interlude-server
```
## Configuration
The application is configured via a `Config.toml` file in the same directory as the binary.
To be able to convert links from and to Spotify and Tidal you need to setup authentication with their API yourself by
providing your `client_id` and `client_secret`.
```toml
# The address on which the server will listen for requests
listen_address_ipv4 = "0.0.0.0"
# The port the server will listen on
listen_port = 5000
# The address under which you want to expose the server to the outside
external_addr = "your.domain.com:443"
# The password used to 'secure' the API. You have to provide it base64 encoded
# in the Authorization Header for each request
api_password = "secret_password"

# Your credentials for the various APIs
[credentials]
tidal = { client_id = "{client_id}", client_secret = "{client_secret}" }
spotify = { client_id = "{client_id}", client_secret = "{client_secret}" }
```

## Usage
To run the server execute the binary. If you have installed the application using 
`cargo install` or added the binary to your `$PATH` you can start the server using the following command:
```bash
interlude
```

Using the default configuration the server will be listening on `0.0.0.0:5000` and will provide the following endpoints:

### Link conversion
`GET /convert?link={share_link}`
#### Example
##### Request
```bash
curl '0.0.0.0:5000/convert' \
-H "Authorization: Bearer $(echo -n 'secret_password' | base64)" \
-G --data-urlencode 'link=https://tidal.com/browse/album/55391786?u'
```
##### Response
```json
{
  "results": [
    {
      "provider": "Spotify",
      "type": "Album",
      "displayName": "The Dark Side of the Moon",
      "url": "https://open.spotify.com/album/4LH4d3cOWNNsVw41Gqt2kv",
      "artwork": "https://i.scdn.co/image/ab67616d00001e02ea7caaff71dea1051d49b2fe"
    },
    {
      "provider": "Tidal",
      "type": "Album",
      "displayName": "The Dark Side of the Moon",
      "url": "https://tidal.com/browse/album/55391786",
      "artwork": "https://resources.tidal.com/images/3009543d/652a/4ab4/ad79/c636323a63cd/320x320.jpg"
    },
    {
      "provider": "Deezer",
      "type": "Album",
      "displayName": "The Dark Side of the Moon",
      "url": "https://www.deezer.com/album/12114240",
      "artwork": "https://cdn-images.dzcdn.net/images/cover/d37e1c39fb5fcd1ead55c4b86e8c610a/250x250-000000-80-0-0.jpg"
    }
  ]
}
```
### Query supported streaming services
`GET /providers`

#### Example
##### Request
```bash
curl '0.0.0.0:5000/providers' \
-H "Authorization: Bearer $(echo -n 'secret_password' | base64)" 
```

##### Response
```bash
[
  {
    "name": "Spotify",
    "url": "https://spotify.com",
    "logoUrl": "your.domain.com:443/public/spotify_logo.png",
    "iconUrl": "your.domain.com:443/public/spotify_icon.png"
  },
  {
    "name": "Tidal",
    "url": "https://tidal.com",
    "logoUrl": "your.domain.com:443/public/tidal_logo.png",
    "iconUrl": "your.domain.com:443/public/tidal_icon.png"
  },
  {
    "name": "Deezer",
    "url": "https://www.deezer.com",
    "logoUrl": "your.domain.com:443/public/deezer_logo.png",
    "iconUrl": "your.domain.com:443/public/deezer_icon.png"
  }
]
```

### Access public files
```
GET /public/{filename}
```
#### Example
##### Request
```bash
curl '0.0.0.0:5000/public/spotify_logo.png' --output spotify_logo.png \
-H "Authorization: Bearer $(echo -n 'secret_password' | base64)" 
```
##### Response
The `spotify_logo.png` file.
