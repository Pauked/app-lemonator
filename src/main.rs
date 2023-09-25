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

fn run() -> eyre::Result<String> {
    color_eyre::install()?;
    log_config::init_log(constants::APP_NAME);
    // This line is intentionally blank... so I can see new runs in the log file
    debug!("");
    debug!(
        "Starting '{}' from '{}', version {}",
        constants::APP_NAME,
        paths::get_current_exe(),
        constants::CRATE_VERSION,
    );

    let args = cli::Args::parse();
    log::debug!("Args {:?}", args);

    let cli_result = cli::run_cli_action(args)?;
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
