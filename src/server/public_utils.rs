use std::fs;
use std::path::Path;

use serde_json::json;

pub fn get_public_files() -> serde_json::Map<std::string::String, serde_json::Value> {
    let public_dir = Path::new("public");
    let mut files = serde_json::Map::new();

    if let Ok(entries) = fs::read_dir(public_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let name = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                let filename = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                files.insert(name, json!(filename));
            }
        }
    }
    files
}
