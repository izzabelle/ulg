use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tile_assets: Vec<String>,
    pub outline: String,
    pub outline_corners: String,
    pub font: String,
}

impl Config {
    pub fn load(path: &str) -> Self {
        let raw = std::fs::read_to_string(path).unwrap();
        toml::from_str(&raw).unwrap()
    }
}
