use clap::{Arg, Command};
use serde::Deserialize;

pub fn parse_args() -> Config {
    let matches = Command::new("Terminal Screensaver")
        .version("0.0.1")
        .author("Your Name")
        .about("Dynamic terminal screen saver with plugin architecture")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets the config file")
                .num_args(1),
        )
        .get_matches();

    let config_path = matches
        .get_one::<String>("config")
        .map(|s| s.as_str())
        .unwrap_or("terminal-screensaver.toml");
    load_config(config_path)
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub text: String,
    pub style: String,
    #[serde(default)]
    pub actions: Vec<ActionConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ActionConfig {
    pub key: String,
    pub description: String,
    pub command: String,
}

fn load_config(path: &str) -> Config {
    let content = std::fs::read_to_string(path).unwrap_or_else(|_| {
        log::warn!("Config file not found, using defaults");
        "".to_string()
    });
    if content.is_empty() {
        Config {
            text: "Welcome to Terminal Screensaver".to_string(),
            style: "default".to_string(),
            actions: Vec::new(),
        }
    } else {
        toml::from_str(&content).unwrap_or_else(|e| {
            log::error!("Failed to parse config: {}", e);
            Config {
                text: "Welcome to Terminal Screensaver".to_string(),
                style: "default".to_string(),
                actions: Vec::new(),
            }
        })
    }
}
