use serde::Deserialize;
use std::fs;
use std::path::PathBuf;


#[derive(Debug, Deserialize)]
pub struct Router{
    #[serde(default)]
    pub sites: Site
}


#[derive(Debug, Deserialize, Default)]
pub struct Site{
    #[serde(default)]
    pub domain: String,
    pub root: String

}




pub fn load_domain() -> Router {
    if let Some(path) = get_config_path() {
        if let Ok(config_text) = fs::read_to_string(&path) {
            if let Ok(router) = toml::from_str::<Router>(&config_text) {
                return router;
            } else {
                eprintln!("Invalid config format, using defaults");
            }
        } else {
            eprintln!("Failed to read config file, using defaults");
        }
    } else {
        eprintln!("No config file found, using defaults");
    }

    Router {
        sites: Site {
            domain: "example.com".to_owned(),
            root: "../rust-server/demo".to_owned()

        },
    }

}




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
