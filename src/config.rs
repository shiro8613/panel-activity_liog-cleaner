use std::fs::File;
use std::io::{BufReader, Write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub weeks_ago :i64,
    pub webhook :String,
    pub database: DatabaseConfig
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: i64,
    pub db_name: String,
    pub username: String,
    pub password: String
}

impl Config {
    pub fn load(path_name :&str) -> Self {
        let file = File::open(path_name).unwrap();
        let reader = BufReader::new(file);

        let conf :Config = serde_yaml::from_reader(reader).unwrap();

        conf
    }

    pub fn create(path_name :&str) {
        let mut file = File::create(path_name).unwrap();

        let data = Self {
            weeks_ago: 4,
            webhook: "https://webhook.discord.com/------".to_string(),
            database: DatabaseConfig {
                host: "127.0.0.1".to_string(),
                port: 3306,
                db_name: "panel".to_string(),
                username: "pterodactyl".to_string(),
                password: "pterodactyl".to_string()
            }
        };

        let config_string = serde_yaml::to_string(&data).unwrap();
        file.write_all(config_string.as_bytes()).unwrap();
    }
}