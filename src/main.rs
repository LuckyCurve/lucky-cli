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
    /// 
    #[clap(subcommand)]
    Website(WebsiteCommand),
    /// 
    Proxy,
}

#[derive(Parser, Debug)]
enum WebsiteCommand {
    List,
    Open {
        /// open index {from command list} url,
        index: usize,
    },
    Add {
        url: String,
    },
    Delete {
        index: usize,
    },
}

impl WebsiteCommand {
    pub async fn execute(&self) {
        let mut config = Config::load().await.unwrap_or_else(|_| Config::default());

        println!("================ {} ================", "WEBSITE".blue());

        match self {
            WebsiteCommand::List => {
                println!("{}\t\t{}", "index", "url");
                config
                    .website
                    .iter()
                    .enumerate()
                    .into_iter()
                    .for_each(|(index, url)| {
                        println!("{}\t\t{}", index + 1, url);
                    })
            }
            WebsiteCommand::Open { index } => {
                if let Some(url) = config.website.get(index - 1) {
                    println!("get url:\t {}", url.to_string().bright_green());
                    open::that(url).unwrap();
                } else {
                    println!("get url error!, index not right");
                }
            }
            WebsiteCommand::Add { url } => {
                if url.starts_with("http://") || url.starts_with("https://") {
                    config.website.push(url.to_string());
                    println!("add url:\t {} success", url);
                    config.save().unwrap();
                } else {
                    println!("input {} {}", url, "error".bright_red());
                }
            }
            WebsiteCommand::Delete { index } => {
                let removed_item = config.website.remove(index - 1);
                println!("delete url:\t {} success", removed_item.to_string());
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
        Command::System.execute().await
    }
}
