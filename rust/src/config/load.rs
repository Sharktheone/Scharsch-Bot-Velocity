use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{ErrorKind, Write};
use crate::config::config_format::{Config, CONFIG_PATH};

pub fn load_config() -> Result<Config, String> {
    let path = Path::new(CONFIG_PATH);

    let mut config_file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                let mut configfile = match File::create(CONFIG_PATH) {
                    Ok(file) => file,
                    Err(e) => {
                        return Err(format!("Error creating config file: {}", e));
                    }
                };

                File::write(&mut configfile, "<TODO: put in standard config>".as_bytes()).unwrap();
                return Err("Config file not found, created new one".to_string());
            } else {
                return Err(format!("Error opening config file: {}", e));
            }
        },
    };


    let mut config_string = String::new();
    match config_file.read_to_string(&mut config_string) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error reading config file: {}", e);
            return Err(format!("Error reading config file: {}", e));
        }
    };

    let config: Config = match serde_json::from_str(&config_string){
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error parsing config file: {}", e);
            return Err(format!("Error parsing config file: {}", e));
        }
    };
    return Ok(config);
}