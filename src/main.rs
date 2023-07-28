use clap::Parser;
use colored::Colorize;

mod cli;
mod constants;
mod db;
mod finder;
mod paths;
mod runner;

const LOG_CONFIG: &str = "logging_config.yaml";

#[tokio::main]
async fn main() {
    //env_logger::init();
    if paths::file_exists(LOG_CONFIG) {
        let log_result = log4rs::init_file(LOG_CONFIG, Default::default());
        match log_result {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to init logging: {:?}", e);
            }
        }
    }
    println!("{}", welcome_to_lemonator());

    let args = cli::Args::parse();
    log::debug!("Args {:?}", args);

    db::create_db().await;

    let result: i32 = cli::run_cli_action(args).await;
    std::process::exit(result);
}

fn welcome_to_lemonator() -> String {
    let mut welcome = String::new();
    welcome.push_str("Welcome to ");
    welcome.push_str("Lemonator".yellow().to_string().as_str());
    welcome.push('!');
    welcome
}
