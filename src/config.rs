use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::system;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub website: HashMap<String, String>,
}

impl Config {
    const PATH: &'static str = "lucky-config/";
    const FILENAME: &'static str = "global.json";

    pub async fn load() -> Result<Config> {
        let file = File::open(system::get_current_user_home_dir() + Self::PATH + Self::FILENAME)?;
        let result: Config = serde_json::from_reader(BufReader::new(file))?;
        Ok(result)
    }

    pub fn save(&self) -> Result<()> {
        let home_dir = system::get_current_user_home_dir();
        fs::create_dir_all(home_dir.clone() + Self::PATH)?;
        let file = File::create(home_dir + Self::PATH + Self::FILENAME)?;
        serde_json::to_writer_pretty(BufWriter::new(file), self)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::Config, system::get_current_user_home_dir};

    #[tokio::test]
    async fn test_config() {
        let config = Config::load().await.unwrap_or_else(|_| {
            println!("load config failed");
            Config::default()
        });

        println!("{:?}", config);

        config.save().unwrap()
    }

    #[test]
    fn test() {
        println!("{}", get_current_user_home_dir())
    }
}
