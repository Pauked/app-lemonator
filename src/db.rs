use crate::{cli::SearchMethod, data};
use chrono::Utc;
use eyre::{Context, Report};
use log::debug;
use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    Sqlite, SqlitePool,
};
use std::{fs, io};

const DB_URL: &str = "sqlite://sqlite.db";
const DB_FILE: &str = "sqlite.db";

static MIGRATOR: Migrator = sqlx::migrate!(); // this will pick up migrations from the ./migrations directory

pub async fn create_db() -> Result<bool, eyre::Report> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        debug!("Creating database {}", DB_URL);
        Sqlite::create_database(DB_URL).await.wrap_err("Unable to create database")?;
        debug!("Create database success");
    } else {
        debug!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let migration_results = MIGRATOR.run(&db).await.wrap_err("Unable to run database migrations")?;
    debug!("Migration success");
    debug!("Migration detail: {:?}", migration_results);
    Ok(true)
}

pub async fn add_app(
    app_name: &str,
    exe_name: &str,
    params: &Option<String>,
    search_term: &str,
    search_method: &SearchMethod,
) -> Result<sqlx::sqlite::SqliteQueryResult, Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

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
    .wrap_err(format!(
        "Failed to add app '{}' with exe_name '{}' and params '{:?}'",
        app_name, exe_name, params
    ))
}

pub async fn get_app(app: &str) -> Result<data::App, Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query_as::<_, data::App>("SELECT * FROM apps WHERE app_name = ? COLLATE NOCASE")
        .bind(app.to_lowercase())
        .fetch_one(&db)
        .await
        .wrap_err(format!("Failed to get app '{}'", app))
}

pub async fn get_apps() -> Result<Vec<data::App>, Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    sqlx::query_as::<_, data::App>("SELECT * FROM apps")
        .fetch_all(&db)
        .await
        .wrap_err("Failed to get list of all apps")
}

pub async fn update_app_path(
    id: i32,
    app_path: &str,
) -> Result<sqlx::sqlite::SqliteQueryResult, Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query("UPDATE apps SET app_path = $1, last_updated = $2 WHERE id=$3 COLLATE NOCASE")
        .bind(app_path)
        .bind(Utc::now())
        .bind(id)
        .execute(&db)
        .await
        .wrap_err(format!(
            "Failed to update app path '{}' for app with id '{}'",
            app_path, id
        ))
}

pub async fn update_last_opened(id: i32) -> Result<sqlx::sqlite::SqliteQueryResult, Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query("UPDATE apps SET last_opened = $1 WHERE id=$2 COLLATE NOCASE")
        .bind(Utc::now())
        .bind(id)
        .execute(&db)
        .await
        .wrap_err(format!("Failed to update last opened for app with id '{}'", id))
}

pub async fn delete_app(app: &str) -> Result<sqlx::sqlite::SqliteQueryResult, Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query("DELETE FROM apps WHERE app_name=$1 COLLATE NOCASE")
        .bind(app.to_lowercase())
        .execute(&db)
        .await
        .wrap_err(format!("Failed to delete app '{}'", app))
}

pub fn reset_db() -> Result<(), io::Error> {
    fs::remove_file(DB_FILE)
}
