use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{ErrorKind, Write};
use crate::config::config_format::{Config, CONFIG_PATH};

pub fn load_config() {
    let path = Path::new(CONFIG_PATH);

    let mut config_file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                let mut configfile = match File::create(CONFIG_PATH) {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("Error creating config file: {}", e);
                        return;
                    }
                };

                File::write(&mut configfile, "<TODO: put in standard config>".as_bytes()).unwrap();
                return;
            } else {
                eprintln!("Error opening config file: {}", e);
                return;
            }
        },
    };


    let mut config_string = String::new();
    match config_file.read_to_string(&mut config_string) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error reading config file: {}", e);
            return;
        }
    };

    let config: Config = serde_json::from_str(&config_string).unwrap();
    println!("Config: {:?}", config);
}