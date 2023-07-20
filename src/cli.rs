use clap::{Parser, Subcommand, ValueEnum};
use strum_macros::EnumString;
use strum_macros::Display;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Opens an app.
    Open {
        app: String,
    },

    /// Adds an app to the database.
    Add {
        /// Nice name for app.
        app: String,
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
        app: String,
    },
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
