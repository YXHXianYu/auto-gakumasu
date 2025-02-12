use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub emulator_name: String,
    pub emulator_path: String,

    pub wait_time_long: f64,
    pub wait_time: f64,

    pub wait_time_start_emulator: f64,
    pub wait_time_start_game_1: f64,
    pub wait_time_start_game_2: f64,

    pub scale_x: f64,
    pub scale_y: f64,

    pub competition_rounds: u64,
}

const ENABLE_SAVE_CONFIG: bool = false;

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "127.0.0.1:5555".to_string(),
            emulator_name: "dnplayer.exe".to_string(),
            emulator_path: r"E:/Programs/leidian/LDPlayerVK/".to_string(),

            wait_time_long: 5.0,
            wait_time: 2.0,

            wait_time_start_emulator: 40.0,
            wait_time_start_game_1: 40.0,
            wait_time_start_game_2: 30.0,

            scale_x: 1.94,
            scale_y: 1.94,

            competition_rounds: 5,
        }
    }
}

pub static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    let config = load_config().unwrap_or_default();
    Mutex::new(config)
});

pub fn get_config() -> Config {
    CONFIG.lock().unwrap().clone()
}

impl Config {
    pub fn get() -> Config {
        CONFIG.lock().unwrap().clone()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {

        if !ENABLE_SAVE_CONFIG {
            return Ok(());
        }

        let config_path = "config.json";
        let config_str = serde_json::to_string_pretty(self)?;
        fs::write(config_path, config_str)?;
        Ok(())
    }

    pub fn update(config: Config) -> Result<(), Box<dyn std::error::Error>> {
        let mut current = CONFIG.lock().unwrap();
        *current = config;
        current.save()?;
        Ok(())
    }
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = "config.json";
    if !Path::new(config_path).exists() {
        let default_config = Config::default();
        default_config.save()?;
        return Ok(default_config);
    }

    let config_str = fs::read_to_string(config_path)?;
    let config: Config = serde_json::from_str(&config_str)?;
    Ok(config)
}