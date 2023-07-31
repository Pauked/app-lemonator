use clap::Parser;
use colored::Colorize;
use log::info;

mod actions;
mod cli;
mod constants;
mod db;
mod finder;
mod paths;
mod log_config;

#[tokio::main]
async fn main() {
    log_config::init_log();
    info!("{}", welcome_to_lemonator());

    let args = cli::Args::parse();
    log::debug!("Args {:?}", args);

    cli::run_cli_action(args).await;
}

fn welcome_to_lemonator() -> String {
    let mut welcome = String::new();
    welcome.push_str("Welcome to ");
    welcome.push_str("Lemonator".yellow().to_string().as_str());
    welcome.push('!');
    welcome
}
