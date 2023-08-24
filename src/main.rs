use clap::Parser;
use log::debug;
use std::process;

mod actions;
mod cli;
mod constants;
mod data;
mod db;
mod finder;
mod log_config;
mod paths;
mod runner;

#[tokio::main]
async fn run() -> eyre::Result<String> {
    color_eyre::install()?;
    log_config::init_log(constants::APP_NAME);
    debug!(
        "Starting '{}' from '{}'",
        constants::APP_NAME,
        paths::get_current_exe()
    );

    let args = cli::Args::parse();
    log::debug!("Args {:?}", args);

    let cli_result = cli::run_cli_action(args).await?;
    Ok(cli_result)
}

fn main() {
    match run() {
        Err(error) => {
            log::error!("Error: {:?}", error);
            process::exit(1);
        }
        Ok(success) => {
            log::info!("{}", success);
            process::exit(0);
        }
    }
}
