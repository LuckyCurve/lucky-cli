use std::collections::BTreeMap;

use arboard::Clipboard;
use colored::Colorize;

use crate::{TemplateCommand, WebsiteCommand};
use crate::config::Config;

pub trait CommandExecute {
    async fn execute(&self);
}

pub trait PrintFormat {
    fn print_header();

    fn print_footer();
}

impl PrintFormat for WebsiteCommand {
    fn print_header() {
        println!("================ {} ================", "WEBSITE".bright_blue());
        println!();
    }

    fn print_footer() {
        println!();
    }
}

impl CommandExecute for WebsiteCommand {
    async fn execute(&self) {
        Self::print_header();

        let mut config = Config::load().await;

        match self {
            WebsiteCommand::List => {
                println!("{}\t\t{}", "key", "url");
                let mut res: Vec<(String, String)> = config.website
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
                    println!("{}\t\t{}", key.bright_green(), url);
                })
            }
            WebsiteCommand::Open { key } => {
                config
                    .website
                    .unwrap_or(BTreeMap::default())
                    .get(key)
                    .map(|url| {
                        println!("get url:\t {}", url.to_string().bright_green());
                        open::that(url).expect(format!("open url error! url: {}", url).as_ref());
                    })
                    .expect("get url error!, index not right");
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
                config.website = Some(config.website.unwrap_or_default().into_iter().filter(|(k, _)| {
                    let contains = k.contains(key);

                    if contains {
                        println!("{}", k.to_owned().bright_green());
                    }

                    !contains
                }).collect());
                config.save().expect("save config error!");
            }
        }

        Self::print_footer();
    }
}

impl PrintFormat for TemplateCommand {
    fn print_header() {
        println!("================ {} ================", "TEMPLATE".bright_blue());
        println!();
    }

    fn print_footer() {
        println!();
    }
}

impl CommandExecute for TemplateCommand {
    async fn execute(&self) {
        Self::print_header();

        let mut config = Config::load().await;
        let mut clipboard = Clipboard::new()
            .expect("get system clipboard error!");
        match self {
            TemplateCommand::List => {
                config.code_template.inspect(|template| {
                    template
                        .keys()
                        .for_each(|item| println!("{}", item.bright_green()))
                });
            }
            TemplateCommand::Choose { key } => {
                let template = config.code_template
                    .unwrap_or_default()
                    .get(key)
                    .expect(format!("code template not exists! code: {}", key).as_str()).to_string();

                clipboard.set_text(template).unwrap();
            }
            TemplateCommand::Add { key } => {
                let code = clipboard.get_text().expect("get clipboard template error!");
                let mut template = config.code_template.unwrap_or_default();
                template.insert(key.to_string(), code);
                config.code_template = Some(template);

                config.save().expect("save config error!");
            }
            TemplateCommand::Remove { key } => {
                config.code_template = Some(config.code_template.unwrap_or_default()
                    .into_iter()
                    .filter(|(k, _)| {
                        let contains = k.contains(key);

                        if contains {
                            println!("{}", k.to_owned().bright_green());
                        }

                        !contains
                    })
                    .collect());

                config.save().expect("save config error!");
            }
            TemplateCommand::Detail { key } => {
                let template = config.code_template
                    .unwrap_or_default()
                    .get(key)
                    .expect(format!("code template not exists! code: {}", key).as_str()).to_string();

                println!("{}", template);
            }
        }

        Self::print_footer();
    }
}
