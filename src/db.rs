use std::{fs, io};

use chrono::{DateTime, Datelike, Local, Timelike, Utc};
use log::debug;
use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    FromRow, Sqlite, SqlitePool,
};
use tabled::Tabled;

use crate::cli::SearchMethod;

const DB_URL: &str = "sqlite://sqlite.db";
const DB_FILE: &str = "sqlite.db";

static MIGRATOR: Migrator = sqlx::migrate!(); // this will pick up migrations from the ./migrations directory

#[derive(Clone, FromRow, Tabled, Debug)]
pub struct App {
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
    //#[tabled(skip)]
    pub app_path: Option<String>,
    #[tabled(
        rename = "Last Opened",
        display_with = "display_option_utc_datetime_to_local"
    )]
    pub last_opened: Option<DateTime<Utc>>,
    #[tabled(
        rename = "Last Updated",
        display_with = "display_option_utc_datetime_to_local"
    )]
    //#[tabled(skip)]
    pub last_updated: Option<DateTime<Utc>>,
}

fn display_option_string(o: &Option<String>) -> String {
    match o {
        Some(s) => s.to_string(),
        None => "N/A".to_string(),
    }
}

fn display_option_utc_datetime_to_local(o: &Option<DateTime<Utc>>) -> String {
    if let Some(o) = o {
        let converted: DateTime<Local> = DateTime::from(*o);
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

pub async fn create_db() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        debug!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => debug!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        debug!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let migration_results = MIGRATOR.run(&db).await;
    match migration_results {
        Ok(_) => {
            debug!("Migration success");
        }
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    debug!("migration: {:?}", migration_results);
}

pub async fn add_app(
    app_name: &str,
    exe_name: &str,
    params: &Option<String>,
    search_term: &str,
    search_method: &SearchMethod,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    // TODO: Check app doesn't exist already

    sqlx::query(
        "INSERT INTO apps (app_name, exe_name, params, search_term, search_method) VALUES (?,?,?,?,?)",
    )
    .bind(app_name)
    .bind(exe_name)
    .bind(params)
    .bind(search_term)
    .bind(search_method.to_string())
    .execute(&db)
    .await
}

pub async fn get_app(app: &str) -> Result<App, sqlx::Error> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query_as::<_, App>("SELECT * FROM apps WHERE app_name = ? COLLATE NOCASE")
        .bind(app.to_lowercase())
        .fetch_one(&db)
        .await
}

pub async fn get_apps() -> Result<Vec<App>, sqlx::Error> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    sqlx::query_as::<_, App>("SELECT * FROM apps")
        .fetch_all(&db)
        .await
}

pub async fn update_app_path(
    id: i32,
    app_path: &str,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query("UPDATE apps SET app_path = $1, last_updated = $2 WHERE id=$3 COLLATE NOCASE")
        .bind(app_path)
        .bind(Utc::now())
        .bind(id)
        .execute(&db)
        .await
}

pub async fn update_last_opened(id: i32) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query("UPDATE apps SET last_opened = $1 WHERE id=$2 COLLATE NOCASE")
        .bind(Utc::now())
        .bind(id)
        .execute(&db)
        .await
}

pub async fn delete_app(app: &str) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query("DELETE FROM apps WHERE app_name=$1 COLLATE NOCASE")
        .bind(app.to_lowercase())
        .execute(&db)
        .await
}

pub fn reset_db() -> Result<(), io::Error> {
    fs::remove_file(DB_FILE)
}
