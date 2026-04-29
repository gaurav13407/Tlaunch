use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub aliases: HashMap<String, String>,
}

pub fn load_config()->Config{
    let path=get_config_path();
    if !Path::new(&path).exists(){
        return Config{
            aliases:HashMap::new(),
        };
    }

    let content=fs::read_to_string(path).unwrap_or_default();

    toml::from_str(&content).unwrap_or(Config{
        aliases:HashMap::new(),
    })
}

pub fn save_config(config: &Config) {
     let dir = format!("{}/.tlaunch", std::env::var("HOME").unwrap());
std::fs::create_dir_all(dir).ok();
    let path = get_config_path();

    let toml = toml::to_string(config).unwrap();
    fs::write(path, toml).expect("Failed to write config");
}

fn get_config_path() -> String {
    let home = std::env::var("HOME").unwrap();
    format!("{}/.tlaunch/config.toml", home)
}
