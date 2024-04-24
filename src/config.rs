use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::system;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub website: Option<BTreeMap<String, String>>,
    pub code_template: Option<HashMap<String, String>>,
}

impl Config {
    const PATH: &'static str = "lucky-config/";
    const FILENAME: &'static str = "global.json";

    pub async fn load() -> Result<Config> {
        let file = File::open(system::get_current_user_home_dir() + Self::PATH + Self::FILENAME)?;
        let result: Config = serde_json::from_reader(BufReader::new(file))?;
        Ok(result)
    }

    pub fn save(&mut self) -> Result<()> {
        self.set_default();

        let home_dir = system::get_current_user_home_dir();
        fs::create_dir_all(home_dir.clone() + Self::PATH)?;
        let file = File::create(home_dir + Self::PATH + Self::FILENAME)?;
        serde_json::to_writer_pretty(BufWriter::new(file), self)?;
        Ok(())
    }

    fn set_default(&mut self) {
        if self.website.is_none() {
            self.website = Some(BTreeMap::default());
        }

        if self.code_template.is_none() {
            self.code_template = Some(HashMap::default());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::Config, system::get_current_user_home_dir};

    #[tokio::test]
    async fn test_config() {
        let config = Config::load().await.unwrap_or_else(|e| {
            println!("load config failed {}", e);
            Config::default()
        });

        println!("{:?}", config);
    }

    #[test]
    fn test() {
        println!("{}", get_current_user_home_dir())
    }
}
