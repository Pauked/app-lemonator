use chrono::{DateTime, Datelike, Local, Timelike, Utc};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tabled::Tabled;

use crate::cli;

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
    pub search_method: String,
    #[tabled(rename = "App Path", display_with = "display_option_string")]
    pub app_path: Option<String>,
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
}

impl App {
    pub fn new(
        app_name: String,
        exe_name: String,
        params: Option<String>,
        search_term: String,
        search_method: String,
    ) -> Self {
        Self {
            id: 0,
            app_name,
            exe_name,
            params,
            search_term,
            search_method,
            app_path: None,
            last_opened: None,
            last_updated: None,
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

        self.search_method
            .parse::<cli::SearchMethod>()
            .map_err(|error| {
                format!(
                    "Invalid search method '{}': {:?}",
                    &self.search_method, error
                )
            })?;

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
