use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};


use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub website: Vec<String>,
}

impl Config {
    const PATH: &'static str = "config/";
    const FILENAME: &'static str = "global.json";

    pub fn load() -> Result<Config, Box<dyn Error>> {
        let file = File::open(Self::PATH.to_owned() + Self::FILENAME)?;
        let result: Config = serde_json::from_reader(BufReader::new(file))?;
        Ok(result)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(Self::PATH)?;
        let file = File::create(Self::PATH.to_owned() + Self::FILENAME)?;
        serde_json::to_writer_pretty(BufWriter::new(file), self)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

    #[test]
    fn test_config() {
        let config = Config::load().unwrap_or_else(|_| {
            println!("load config failed");
            Config::default()
        });

        println!("{:?}", config);

        config.save().unwrap()
    }
}