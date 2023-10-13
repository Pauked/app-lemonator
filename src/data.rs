use chrono::{DateTime, Datelike, Local, Timelike, Utc};
use clap::ValueEnum;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum_macros::Display;
use strum_macros::EnumString;
use tabled::Tabled;

use crate::finder;

#[derive(Clone, Debug)]
pub struct FileVersion {
    pub app_description: String,
    pub path: String,
    pub major: u32,
    pub minor: u32,
    pub build: u32,
    pub revision: u32,
}

impl FileVersion {
    pub fn new(
        app_description: String,
        path: String,
        version: String,
    ) -> Self {
        match finder::parse_file_version_to_i32(&version) {
            Some((major, minor, build, revision)) => {
                Self {
                    app_description,
                    path,
                    major,
                    minor,
                    build,
                    revision,
                }
            }
            None => {
                Self {
                    app_description,
                    path,
                    major: 0,
                    minor: 0,
                    build: 0,
                    revision: 0,
                }
            }
        }
    }

    pub fn display_version(&self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.major, self.minor, self.build, self.revision
        )
    }
}

#[derive(
    ValueEnum, Clone, Serialize, Deserialize, Debug, Display, EnumString, PartialEq, sqlx::Type,
)]
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

#[derive(
    ValueEnum, Clone, Debug, Serialize, Deserialize, Display, EnumString, PartialEq, sqlx::Type,
)]
pub enum OperatingSystem {
    /// Windows
    #[value(alias("Windows"))]
    Windows,
    /// MacOs
    #[value(alias("MacOS"))]
    MacOS,
    /// Unknown! Sorry Linux.
    #[value(alias("Unknown"))]
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow, Tabled)]
pub struct App {
    #[serde(skip)]
    #[tabled(skip)]
    pub id: i32,
    #[tabled(rename = "App Name")]
    pub app_name: String,
    #[tabled(rename = "Exe Name")]
    pub exe_name: String,
    #[tabled(rename = "Params", display_with = "display_option_string")]
    pub params: Option<String>,
    #[tabled(rename = "Search Term")]
    pub search_term: String,
    #[tabled(rename = "Search Method")]
    pub search_method: SearchMethod,
    #[tabled(rename = "App Path", display_with = "display_option_string")]
    pub app_path: Option<String>,
    #[tabled(rename = "App Description", display_with = "display_option_string")]
    pub app_description: Option<String>,
    #[tabled(rename = "App Version", display_with = "display_option_string")]
    pub app_version: Option<String>,
    #[serde(skip)]
    #[tabled(
        rename = "Last Opened",
        display_with = "display_option_utc_datetime_to_local"
    )]
    pub last_opened: Option<DateTime<Utc>>,
    #[serde(skip)]
    #[tabled(
        rename = "Last Updated",
        display_with = "display_option_utc_datetime_to_local"
    )]
    pub last_updated: Option<DateTime<Utc>>,
    #[tabled(rename = "Operating System")]
    pub operating_system: OperatingSystem,
}

impl App {
    pub fn new(
        app_name: String,
        exe_name: String,
        params: Option<String>,
        search_term: String,
        search_method: SearchMethod,
        operating_system: OperatingSystem,
    ) -> Self {
        Self {
            id: 0,
            app_name,
            exe_name,
            params,
            search_term,
            search_method,
            app_path: None,
            app_description: None,
            app_version: None,
            last_opened: None,
            last_updated: None,
            operating_system,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.app_name.is_empty() {
            return Err("App Name is empty.".to_owned());
        }
        if self.exe_name.is_empty() {
            return Err("Exe Name is empty.".to_owned());
        }
        if self.search_term.is_empty() {
            return Err("Search Term is empty.".to_owned());
        }
        if self.search_method == SearchMethod::PSGetApp
            && self.operating_system != OperatingSystem::Windows
        {
            return Err(format!(
                "Search method '{}' is only supported on Windows",
                &self.search_method
            ));
        }

        Ok(())
    }

    pub fn to_description(&self) -> String {
        let param_info = if let Some(unwrapped_params) = self.params.clone() {
            format!(" Params '{}'", unwrapped_params.magenta())
        } else {
            String::new()
        };

        format!(
            "App Name '{}', Exe Name '{}', Search Term '{}', Search Method '{}'{}",
            self.app_name.blue(),
            self.exe_name.magenta(),
            self.search_term.magenta(),
            self.search_method.to_string().magenta(),
            param_info
        )
    }
}

pub fn display_option_string(value: &Option<String>) -> String {
    match value {
        Some(s) => s.to_string(),
        None => "N/A".to_string(),
    }
}

pub fn display_option_utc_datetime_to_local(value: &Option<DateTime<Utc>>) -> String {
    if let Some(d) = value {
        let converted: DateTime<Local> = DateTime::from(*d);
        return format_local_datetime(&converted);
    }

    "N/A".to_string()
}

fn format_local_datetime(local_datetime: &DateTime<Local>) -> String {
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        local_datetime.year(),
        local_datetime.month(),
        local_datetime.day(),
        local_datetime.hour(),
        local_datetime.minute(),
        local_datetime.second()
    )
}

#[cfg(test)]
mod tests {
    use super::{App, OperatingSystem, SearchMethod};

    #[test]
    fn app_validate_psgetapp_on_macos_fails() {
        // Arrange
        let new_app = App::new(
            "app_name".to_string(),
            "exe_name".to_string(),
            None,
            "search_term".to_string(),
            SearchMethod::PSGetApp,
            OperatingSystem::MacOS,
        );

        // Act
        let actual = new_app.validate();

        // Assert
        assert!(actual.is_err());
        assert!(actual.unwrap_err().contains(&format!(
            "Search method '{}' is only supported on Windows",
            &new_app.search_method
        )));
    }

    #[test]
    fn app_validate_no_app_name() {
        // Arrange
        let new_app = App::new(
            "".to_string(),
            "exe_name".to_string(),
            None,
            "search_term".to_string(),
            SearchMethod::FolderSearch,
            OperatingSystem::Windows,
        );

        // Act
        let actual = new_app.validate();

        // Assert
        assert!(actual.is_err());
        assert!(actual.unwrap_err().contains("App Name is empty."));
    }

    #[test]
    fn app_validate_no_exe_name() {
        // Arrange
        let new_app = App::new(
            "app_name".to_string(),
            "".to_string(),
            None,
            "search_term".to_string(),
            SearchMethod::FolderSearch,
            OperatingSystem::Windows,
        );

        // Act
        let actual = new_app.validate();

        // Assert
        assert!(actual.is_err());
        assert!(actual.unwrap_err().contains("Exe Name is empty."));
    }

    #[test]
    fn app_validate_no_search_term() {
        // Arrange
        let new_app = App::new(
            "app_name".to_string(),
            "exe_name".to_string(),
            None,
            "".to_string(),
            SearchMethod::FolderSearch,
            OperatingSystem::Windows,
        );

        // Act
        let actual = new_app.validate();

        // Assert
        assert!(actual.is_err());
        assert!(actual.unwrap_err().contains("Search Term is empty."));
    }

    #[test]
    fn app_validate_psgetapp_on_windows_success() {
        // Arrange
        let new_app = App::new(
            "app_name".to_string(),
            "exe_name".to_string(),
            None,
            "search_term".to_string(),
            SearchMethod::PSGetApp,
            OperatingSystem::Windows,
        );

        // Act
        let actual = new_app.validate();

        // Assert
        assert!(actual.is_ok());
    }
}
