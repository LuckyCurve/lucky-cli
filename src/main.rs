use clap::Parser;
use colored::Colorize;

use crate::service::CommandExecute;
use crate::system::Sys;

mod config;
mod network;
mod service;
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
    #[clap(subcommand, alias = "temp")]
    Template(TemplateCommand),
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
    /// remove website that you have added, can be shortened as "d"
    #[command(alias = "r")]
    Remove { key: String },
}

#[derive(Debug, Parser)]
enum TemplateCommand {
    /// show all templates, can be shortened as "l"
    #[command(alias = "l")]
    List,
    #[command(alias = "c")]
    Choose { key: String },
    #[command(alias = "a")]
    Add { key: String },
    #[command(alias = "r")]
    Remove { key: String },
    #[command(alias = "d")]
    Detail { key: String },
}

impl Command {
    async fn execute(&self) {
        match self {
            Command::IP => {
                let public_ip_future = tokio::spawn(async { network::get_public_ip().await });
                let local_ip_future = tokio::spawn(async { network::get_local_ip().await });
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
            Command::Template(template) => {
                template.execute().await;
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
