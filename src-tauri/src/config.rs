#[derive(serde::Deserialize)]
pub struct Config { 
    pub media: MediaConfig, 
}

#[derive(serde::Deserialize)]
pub struct MediaConfig { pub tmdb_key: String, }

impl Config {
    pub fn get_config() -> Config {
        let config = std::fs::read_to_string("config.json").unwrap();
        serde_json::from_str::<Config>(&config).unwrap()
    }
}