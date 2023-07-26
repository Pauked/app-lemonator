use clap::Parser;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm};
use tabled::settings::{object::Rows, Modify, Width};

mod cli;
mod db;
mod finder;
mod paths;
mod runner;

#[tokio::main]
async fn main() {
    //env_logger::init();
    let log_result = log4rs::init_file("logging_config.yaml", Default::default());
    match log_result {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to init logging: {:?}", e);
        }
    }
    println!("{}", welcome_to_lemonator());

    let args = cli::Args::parse();
    log::debug!("Args {:?}", args);

    db::create_db().await;

    match args.action {
        cli::Action::Open { app_name } => match db::get_app(&app_name).await {
            Ok(app) => {
                runner::run_app(app).await;
            }
            Err(_) => {
                log::error!("App '{}' not found", app_name);
            }
        },
        cli::Action::Add {
            app_name,
            exe_name,
            search_term,
            search_method,
        } => {
            db::add_app(&app_name, &exe_name, &search_term, search_method).await;
        }
        cli::Action::Delete { app_name } => {
            db::delete_app(&app_name).await;
        }
        cli::Action::List {} => {
            let apps = db::get_apps().await;
            println!("{}", "App Listing".blue());
            println!(
                "{}",
                tabled::Table::new(apps)
                    .with(Modify::new(Rows::new(1..)).with(Width::wrap(30).keep_words()))
            );
        }
        cli::Action::Reset {} => {
            // Prompt the user for confirmation to delete the file
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to reset the database? All data will be deleted.")
                .interact()
                .unwrap()
            {
                db::reset_db();
            } else {
                println!("Deletion not confirmed.");
            }
        }
        cli::Action::Testings {} => {
            println!("Testing!");
        }
    }
}

fn welcome_to_lemonator() -> String {
    let mut welcome = String::new();
    welcome.push_str("Welcome to ");
    welcome.push_str("Lemonator".yellow().to_string().as_str());
    welcome.push('!');
    welcome
}
