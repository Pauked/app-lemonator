use chrono::{DateTime, Utc, Local, Datelike, Timelike};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use tabled::Tabled;

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