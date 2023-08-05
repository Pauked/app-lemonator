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

const APP_NAME: &str = "app-lemonator";

#[tokio::main]
async fn main() {
    log_config::init_log(APP_NAME);
    info!("{}", welcome_to_lemonator());

    let args = cli::Args::parse();
    log::debug!("Args {:?}", args);

    cli::run_cli_action(args).await;
}

fn welcome_to_lemonator() -> String {
    let mut welcome = String::new();
    welcome.push_str("Welcome to ");
    welcome.push_str("App-Lemonator".yellow().to_string().as_str());
    /*
    welcome.push_str(r#"                         _                                  _             "#);
    welcome.push_str(r#"                        | |                                | |            "#);
    welcome.push_str(r#"  __ _ _ __  _ __ ______| | ___ _ __ ___   ___  _ __   __ _| |_ ___  _ __ "#);
    welcome.push_str(r#" / _` | '_ \| '_ \______| |/ _ \ '_ ` _ \ / _ \| '_ \ / _` | __/ _ \| '__|"#);
    welcome.push_str(r#"| (_| | |_) | |_) |     | |  __/ | | | | | (_) | | | | (_| | || (_) | |   "#);
    welcome.push_str(r#" \__,_| .__/| .__/      |_|\___|_| |_| |_|\___/|_| |_|\__,_|\__\___/|_|   "#);
    welcome.push_str(r#"      | |   | |                                                           "#);
    welcome.push_str(r#"      |_|   |_|                                                           "#);
    */
    welcome.push('!');
    welcome
}



