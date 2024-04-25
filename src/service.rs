use std::collections::BTreeMap;

use colored::Colorize;

use crate::config::Config;
use crate::{TemplateCommand, WebsiteCommand};

pub trait CommandExecute {
    async fn execute(&self);
}

impl CommandExecute for WebsiteCommand {
    async fn execute(&self) {
        let mut config = Config::load().await;

        println!("================ {} ================", "WEBSITE".blue());

        match self {
            WebsiteCommand::List => {
                println!("{}\t\t{}", "key", "url");
                let mut res: Vec<(String, String)> = config
                    .website
                    .unwrap_or(BTreeMap::default())
                    .into_iter()
                    .collect();

                res.sort_by(|(key1, _), (key2, _)| {
                    let result1 = key1.parse::<i32>();
                    let result2 = key2.parse::<i32>();

                    result1
                        .and_then(|value1| result2.and_then(|value2| Ok(value1.cmp(&value2))))
                        .unwrap_or(key1.cmp(&key2))
                });

                res.iter().for_each(|(key, url)| {
                    println!("{}\t\t{}", key, url);
                })
            }
            WebsiteCommand::Open { key } => {
                config
                    .website
                    .unwrap_or(BTreeMap::default())
                    .get(key)
                    .map(|url| {
                        println!("get url:\t {}", url.to_string().bright_green());
                        open::that(url).unwrap_or_else(|e| {
                            panic!("open url: {} error!\n error info {}", url, e);
                        });
                    })
                    .unwrap_or_else(|| {
                        panic!("get url error!, index not right");
                    });
            }
            WebsiteCommand::Add { key, url } => {
                if url.starts_with("http://") || url.starts_with("https://") {
                    let mut website = config.website.unwrap_or_default();

                    println!("add url:\t {} success", url);
                    if let Some(removed_url) = website.insert(key.to_owned(), url.to_owned()) {
                        println!("removed url:\t {} success", removed_url);
                    }

                    config.website = Some(website);
                    config.save().unwrap();
                } else {
                    println!("input {} {}", url, "error".bright_red());
                }
            }
            WebsiteCommand::Remove { key } => {
                let mut website = config.website.unwrap_or_default();

                if let Some(removed_url) = website.remove(key) {
                    println!("delete url:\t {} success", removed_url.to_string());
                }

                config.website = Some(website);
                config.save().unwrap();
            }
        }
    }
}

impl CommandExecute for TemplateCommand {
    async fn execute(&self) {
        let config = Config::load().await;
        match self {
            TemplateCommand::List => {
                config.code_template.inspect(|template| {
                    template
                        .keys()
                        .for_each(|item| println!("{}", item.bright_green()))
                });
            }
            TemplateCommand::Choose { .. } => {}
            TemplateCommand::Add { .. } => {}
            TemplateCommand::Remove { .. } => {}
            TemplateCommand::Detail { .. } => {}
        }
    }
}
