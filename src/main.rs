use clap::Parser;
use color_eyre::eyre::Result;
use colored::Colorize;
use log::{debug, info};

mod actions;
mod cli;
mod constants;
mod db;
mod finder;
mod log_config;
mod paths;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    log_config::init_log(constants::APP_NAME);
    debug!(
        "Starting '{}' from '{}'",
        constants::APP_NAME,
        paths::get_current_exe()
    );
    info!("{}", welcome_to_lemonator());

    let args = cli::Args::parse();
    log::debug!("Args {:?}", args);

    cli::run_cli_action(args).await;

    Ok(())
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
