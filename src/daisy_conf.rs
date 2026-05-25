use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub server: Server,
}

#[derive(Debug, Deserialize, Default)]
pub struct Server {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_base")]
    pub base: String,
}

// -------- Defaults --------

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    3000
}

fn default_base() -> String {
    "./public".to_string()
}

// -------- Loader --------

pub fn load_config() -> Config {
    if let Some(path) = get_config_path() {
        if let Ok(config_text) = fs::read_to_string(&path) {
            if let Ok(config) = toml::from_str::<Config>(&config_text) {
                return config;
            } else {
                eprintln!("Invalid config format, using defaults");
            }
        } else {
            eprintln!("Failed to read config file, using defaults");
        }
    } else {
        eprintln!("No config file found, using defaults");
    }

    // fallback
    Config {
        server: Server {
            host: default_host(),
            port: default_port(),
            base: default_base(),
        },
    }
}

// -------- Path Resolution --------

fn get_config_path() -> Option<PathBuf> {
    // 1. Local config
    let local = PathBuf::from("config.toml");
    if local.exists() {
        return Some(local);
    }

    // 2. Global config
    if let Some(dir) = dirs::config_dir() {
        let global = dir.join("daisy").join("config.toml");
        if global.exists() {
            return Some(global);
        }
    }

    None
}


