use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use strum_macros::Display;
use strum_macros::EnumString;
use tabled::settings::object::Rows;
use tabled::settings::Modify;
use tabled::settings::Width;

use crate::db;
use crate::finder;
use crate::runner;

#[derive(Parser, Debug)]
#[command(
    name = "App Lemonator",
    version = "0.1.0",
    author = "Paul",
    about = "Keeps the running of your apps lemony fresh!"
)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Opens an app.
    #[clap(short_flag='o')]
    Open { app_name: String },

    /// Adds an app to the database.
    #[clap(short_flag='a')]
    Add {
        /// Nice name for app.
        app_name: String,
        /// Executable to find and run. For Shortcuts, can be full path and exe.
        exe_name: String,
        /// Search text for app.
        search_term: String,
        /// Search method to find app.
        #[clap(value_enum)]
        search_method: SearchMethod,
    },

    /// Deletes the app from the database.
    #[clap(short_flag='d')]
    Delete { app_name: String },

    /// Update the running folder for selected apps. No app named means all in database.
    #[clap(short_flag='u')]
    Update {
        /// App name to update.
        app_name: Option<String>,
    },

    /// Lists all apps in the database.
    #[clap(short_flag='l')]
    List,

    /// Resets the database.
    #[clap(short_flag='r')]
    Reset,

    /// Testings, sssssh.
    Testings,
}

#[derive(ValueEnum, Clone, Debug, Display, EnumString)]
pub enum SearchMethod {
    /// Uses PowerShell to run the Get-AppXPackage cmdlet to retrieve InstallLocation.
    #[value(alias("PSGetApp"))]
    PSGetApp,
    /// Given a root folder, it will recursively search for the app.
    #[value(alias("FolderSearch"))]
    FolderSearch,
    /// Just runs the app directly. No lookups, you provide the full path.
    #[value(alias("Shortcut"))]
    Shortcut,
}

pub async fn run_cli_action(args: Args) -> i32 {
    //let mut exit_code = 0;

    match args.action {
        Action::Open { app_name } => {
            let app = db::get_app(&app_name).await;
            println!("Running app '{}'", app.app_name);
            runner::run_app(app).await;
            // TODO: If app not found, error code?
        },
        Action::Add {
            app_name,
            exe_name,
            search_term,
            search_method,
        } => {
            db::add_app(&app_name, &exe_name, &search_term, search_method).await;
        }
        Action::Delete { app_name } => {
            db::delete_app(&app_name).await;
        }
        Action::Update { app_name } => match app_name {
            Some(app_name) => {
                let app = db::get_app(&app_name).await;
                finder::update_app(app).await;
            }
            None => {
                let apps = db::get_apps().await;
                finder::update_all_apps(apps).await;
            }
        },
        Action::List {} => {
            let apps = db::get_apps().await;
            println!("{}", "App Listing".blue());
            println!(
                "{}",
                tabled::Table::new(apps)
                    .with(Modify::new(Rows::new(1..)).with(Width::wrap(30).keep_words()))
            );
        }
        Action::Reset {} => {
            // Prompt the user for confirmation to delete the file
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to reset the database? All data will be deleted.")
                .interact()
                .unwrap()
            {
                db::reset_db();
            } else {
                println!("Reset not confirmed.");
            }
        }
        Action::Testings {} => {
            println!("Testing!");
            let app = db::get_app("rider").await;
            finder::testings_progress(app);
        }
    }

    //exit_code
    0
}
