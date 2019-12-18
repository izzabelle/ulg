use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tile_assets: Vec<String>,
}

impl Config {
    pub fn load(path: &str) -> Self {
        let raw = std::fs::read_to_string(path).unwrap();
        toml::from_str(&raw).unwrap()
    }
}
