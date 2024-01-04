use std::fs;
use std::io::{self, Error, ErrorKind};

pub fn run_configuration_setup() -> io::Result<()> {
    // Read the configuration file
    let config_contents = fs::read_to_string("config.toml")?;

    // Print the lines to help debug
    for line in config_contents.lines() {
        println!("Line from config file: {:?}", line);
    }

    // Default configurations
    let mut auth_enabled = false;
    let mut database_enabled = false;

    // Parse the configuration
    for line in config_contents.lines() {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            return Err(Error::new(ErrorKind::InvalidData, "Malformed config line"));
        }

        match parts[0].trim() {
            "auth_enabled" => auth_enabled = parts[1].trim() == "true",
            "database_enabled" => database_enabled = parts[1].trim() == "true",
            _ => return Err(Error::new(ErrorKind::InvalidData, "Unknown config key")),
        }
    }

    // Initialize features based on configuration
    if auth_enabled {
        // Initialize auth module
        println!("Auth module initialized ");
    }

    if database_enabled {
        // Initialize database module
        println!("Database module initialized");
    
    }
    // TODO:now actually initialize :D & expand for other services. some may be dependent on others such as the cms and database therefore there will be defaults that
    // are applied, such as the http server, and routing system.
    


    Ok(())
}