# Interlude Server

Ein Rust-basierter HTTP-Server, der Music-Streaming-Links zwischen verschiedenen Plattformen konvertiert (Spotify, Tidal, Apple Music, Deezer).

## 📋 Setup

### Konfiguration
1. Kopiere `Config.toml.example` nach `Config.toml`
2. Fülle deine API-Credentials für die verschiedenen Streaming-Services aus
3. Setze ein sicheres `api_password`

### 🐳 Docker Setup (Empfohlen)

```bash
# Docker Compose verwenden (Port 30002)
docker-compose up -d

# Oder manuell:
docker build -t interlude-server .
docker run -p 30002:5000 -v ./Config.toml:/app/Config.toml:ro interlude-server
```

### 🦀 Lokale Entwicklung

**Wichtig:** Für lokale Entwicklung muss der Bind-Address geändert werden:

```rust
// In src/main.rs Zeile 19:
let addr = SocketAddr::from(([127, 0, 0, 1], PORT)); // Für lokale Entwicklung
// let addr = SocketAddr::from(([0, 0, 0, 0], PORT)); // Für Docker
```

```bash
# Projekt starten
cargo run
```

## 🚀 API Usage

### Authentication
Alle Requests benötigen einen `Authorization` Header mit Base64-kodiertem API-Passwort:

```bash
# API-Passwort Base64 kodieren
echo -n "dein_api_passwort" | base64
```

### Endpoints

#### Link Konvertierung
```
GET /convert?link={music_link}
```

**Headers:**
```
Authorization: Basic {base64_encoded_api_password}
```

#### Verfügbare Plattformen abfragen
```
GET /platforms
```

**Headers:**
```
Authorization: Basic {base64_encoded_api_password}
```

**Response Format:**
```json
{
  "Spotify": "Spotify.jpeg",
  "Tidal": "Tidal.jpeg"
}
```

#### Plattform-Dateien abrufen
```
GET /public/{filename}
```

**Headers:**
```
Authorization: Basic {base64_encoded_api_password}
```

**Beispiele:**
```bash
# API-Passwort kodieren (Beispiel: "mein_secret")
API_TOKEN=$(echo -n "mein_secret" | base64)

# Spotify Link konvertieren
curl -H "Authorization: Basic $API_TOKEN" \
     "http://localhost:30002/convert?link=https://open.spotify.com/track/4iV5W9uYEdYUVa79Axb7Rh"

# Tidal Link konvertieren  
curl -H "Authorization: Basic $API_TOKEN" \
     "http://localhost:30002/convert?link=https://tidal.com/browse/track/123456789"

# Apple Music Link konvertieren
curl -H "Authorization: Basic $API_TOKEN" \
     "http://localhost:30002/convert?link=https://music.apple.com/album/track/123456789"

# Verfügbare Plattformen abfragen
curl -H "Authorization: Basic $API_TOKEN" \
     "http://localhost:30002/platforms"

# Plattform-Logo abrufen
curl -H "Authorization: Basic $API_TOKEN" \
     "http://localhost:30002/public/Spotify.jpeg" -o spotify_logo.jpeg
```

**Response Format:**
```json
{
  "title": "Song Title",
  "artist": "Artist Name",
  "album": "Album Name",
  "links": {
    "spotify": "https://open.spotify.com/track/...",
    "tidal": "https://tidal.com/browse/track/...",
    "apple_music": "https://music.apple.com/...",
    "deezer": "https://deezer.com/track/..."
  }
}
```

## 🔧 Unterstützte Plattformen

- ✅ **Spotify** - Tracks, Alben, Playlisten
- ✅ **Tidal** - Tracks, Alben  
- ✅ **Apple Music** - Tracks, Alben
- ✅ **Deezer** - Tracks, Alben

## 📁 Zusätzliche Features

- 🏷️ **Plattform-Abfrage** - Abrufen verfügbarer Plattformen über `/platforms` Endpoint
- 🖼️ **Statische Dateien** - Servieren von Plattform-Logos und anderen Assets über `/public/{filename}`
- 🔒 **Sicherheit** - Alle Endpoints erfordern Authentifizierung