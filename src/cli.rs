use clap::{Parser, ValueEnum};

use strum_macros::Display;
use strum_macros::EnumString;

use crate::actions;

// https://github.com/clap-rs/clap/blob/master/examples/git-derive.rs
// https://github.com/glotlabs/gdrive/blob/main/src/main.rs
#[derive(Parser, Debug, PartialEq)]
#[command(
    name = "App Lemonator",
    version = "0.1.0",
    author = "Paul",
    about = "Keeps the running of your apps lemony fresh!"
)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
}

// #[derive(Subcommand, Debug, PartialEq)]
#[derive(Parser, Debug, PartialEq)]
//#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Action {
    /// Opens an app.
    #[clap(short_flag = 'o')]
    Open { app_name: String },

    /// Adds an app to the database.
    #[clap(short_flag = 'a')]
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
        /// Parameters to pass to app.
        params: Option<Vec<String>>,
    },

    /// Deletes the app from the database.
    #[clap(short_flag = 'd')]
    Delete { app_name: String },

    /// Update the running folder for selected apps. No app named means all in database.
    #[clap(short_flag = 'u')]
    Update {
        /// App name to update.
        app_name: Option<String>,
    },

    /// Lists all apps in the database.
    #[clap(short_flag = 'l')]
    List {
        /// Individual app name to list.
        app_name: Option<String>,
    },

    /// Resets the database.
    #[clap(short_flag = 'r')]
    Reset,

    /// Testings, sssssh.
    Testings,
}

#[derive(ValueEnum, Clone, Debug, Display, EnumString, PartialEq)]
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

pub async fn run_cli_action(args: Args) {
    if args.action != Action::Reset {
        actions::create_db().await;
    }

    match args.action {
        Action::Open { app_name } => {
            actions::open_app(&app_name).await;
        }
        Action::Add {
            app_name,
            exe_name,
            params,
            search_term,
            search_method,
        } => {
            actions::add_app(
                app_name,
                exe_name,
                params.map(|p| p.join(" ")),
                search_term,
                search_method,
            )
            .await;
        }
        Action::Delete { app_name } => {
            actions::delete_app(&app_name).await;
        }
        Action::Update { app_name } => {
            actions::update_app(app_name).await;
        }
        Action::List { app_name } => {
            actions::list_app(app_name).await;
        }
        Action::Reset {} => actions::reset(),
        Action::Testings {} => actions::testings(),
    }
}
