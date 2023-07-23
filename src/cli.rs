use clap::{Parser, Subcommand, ValueEnum};
use strum_macros::EnumString;
use strum_macros::Display;

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
    Open {
        app_name: String,
    },

    /// Adds an app to the database.
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
    Delete {
        app_name: String,
    },

    /// Lists all apps in the database.
    List,

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
