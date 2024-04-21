use clap::Parser;
use colored::Colorize;
use sysproxy::Sysproxy;

use crate::config::Config;
use crate::system::Sys;

mod config;
mod network;
mod system;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    /// get current machine ip, including all derives ip
    IP,
    /// get current machine infomation
    System,
    /// manager your favorite site in one place! can be shortened with "web"
    #[clap(subcommand, alias = "web")]
    Website(WebsiteCommand),
    /// see your current machine proxy infomation
    Proxy,
}

#[derive(Parser, Debug)]
enum WebsiteCommand {
    /// render website that you favourite, can be shortened as "l"
    #[command(alias = "l")]
    List,
    /// open website that you favourite, can be shortened as "o"
    #[command(alias = "o")]
    Open {
        /// open index {from command list} url,
        key: String,
    },
    /// add website that you favourite, can be shortened as "a"
    #[command(alias = "a")]
    Add { key: String, url: String },
    /// delete website that you have added, can be shortened as "d"
    #[command(alias = "d")]
    Delete { key: String },
}

impl WebsiteCommand {
    pub async fn execute(&self) {
        let mut config = Config::load().await.unwrap_or_else(|_| Config::default());

        println!("================ {} ================", "WEBSITE".blue());

        match self {
            WebsiteCommand::List => {
                println!("{}\t\t{}", "key", "url");
                let mut res: Vec<(String, String)> = config.website.into_iter().collect();

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
                if let Some(url) = config.website.get(key) {
                    println!("get url:\t {}", url.to_string().bright_green());
                    open::that(url).unwrap();
                } else {
                    println!("get url error!, index not right");
                }
            }
            WebsiteCommand::Add { key, url } => {
                if url.starts_with("http://") || url.starts_with("https://") {
                    let removed_item = config.website.insert(key.to_owned(), url.to_owned());
                    println!("add url:\t {} success", url);
                    if let Some(removed_url) = removed_item {
                        println!("removed url:\t {} success", removed_url);
                    }

                    config.save().unwrap();
                } else {
                    println!("input {} {}", url, "error".bright_red());
                }
            }
            WebsiteCommand::Delete { key } => {
                let removed_item = config.website.remove(key);
                if let Some(removed_url) = removed_item {
                    println!("delete url:\t {} success", removed_url.to_string());
                }
                config.save().unwrap();
            }
        }
    }
}

impl Command {
    async fn execute(&self) {
        match self {
            Command::IP => {
                let public_ip_future = tokio::spawn(async move { network::get_public_ip().await });
                let local_ip_future = tokio::spawn(async move { network::get_local_ip().await });
                println!("================ {} ================", "CURRENT IP".blue());
                println!(
                    "public ip:\t{}",
                    public_ip_future
                        .await
                        .unwrap()
                        .unwrap()
                        .bright_green()
                        .bold()
                );
                println!(
                    "local ip:\t{}",
                    local_ip_future
                        .await
                        .unwrap()
                        .unwrap()
                        .bright_green()
                        .bold()
                );
            }
            Command::System => {
                let mut sys = Sys::new();

                let memory = sys.memory_usage().await;
                println!("================ {} ================", "SYSTEM".blue());
                println!("{}", "memory:".to_string().blue());
                println!(
                    "available memory:\t{}",
                    memory.get_available_memory().bright_green()
                );
                println!(
                    "total memory:\t\t{}",
                    memory.get_total_memory().bright_green()
                );
                println!(
                    "percentage usage:\t{}",
                    memory.get_percent_usage().bright_green()
                );
                println!();

                let cpu = sys.average_cpu_usage().await;
                let cpu_usage = format!("{:.2}%", cpu.cpu_usage());
                println!("{}", "cpu:".to_string().blue());
                println!("average cpu usage:\t{}", cpu_usage.bright_green());
            }
            Command::Website(website_command) => {
                website_command.execute().await;
            }
            Command::Proxy => {
                let result = Sysproxy::get_system_proxy().unwrap();
                let sys_proxy = format!("{:?}", result);
                println!("{}", sys_proxy.bright_green());
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    cli.command.execute().await
}

#[cfg(test)]
mod tests {
    use crate::Command;

    #[tokio::test]
    async fn test_main() {
        Command::Website(crate::WebsiteCommand::Add {
            key: "1".to_string(),
            url: "http://www.baidu,com".to_string(),
        })
        .execute()
        .await;
    }
}
