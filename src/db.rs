use crate::data;
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

pub async fn database_exists() -> bool {
    Sqlite::database_exists(DB_URL).await.unwrap_or(false)
}

pub async fn create_db() -> Result<bool, eyre::Report> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        debug!("Creating database {}", DB_URL);
        Sqlite::create_database(DB_URL)
            .await
            .wrap_err("Unable to create database")?;
        debug!("Successfully created database");
    } else {
        debug!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();
    MIGRATOR
        .run(&db)
        .await
        .wrap_err("Unable to run database migrations")?;
    debug!("Migration success");
    Ok(true)
}

pub async fn add_app(app: &data::App) -> Result<sqlx::sqlite::SqliteQueryResult, Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query(
        "INSERT INTO apps (app_name, exe_name, params, search_term, search_method, operating_system) VALUES (?,?,?,?,?,?)",
    )
    .bind(&app.app_name)
    .bind(&app.exe_name)
    .bind(&app.params)
    .bind(&app.search_term)
    .bind(&app.search_method)
    .bind(&app.operating_system)
    .execute(&db)
    .await
    .wrap_err(format!(
        "Failed to add app '{:?}", app
    ))
}

pub async fn edit_app(
    lookup_app_name: &str,
    app: &data::App,
) -> Result<sqlx::sqlite::SqliteQueryResult, Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query("UPDATE apps SET app_name=$1, exe_name=$2, search_term=$3, search_method=$4, params=$5 WHERE app_name=$6 COLLATE NOCASE")
    .bind(&app.app_name)
    .bind(&app.exe_name)
    .bind(&app.search_term)
    .bind(&app.search_method)
    .bind(&app.params)
    .bind(lookup_app_name)
    .execute(&db)
    .await
    .wrap_err(format!(
        "Failed to edit app '{:?}'", app
    ))
}

pub async fn get_app(app: &str) -> Result<data::App, Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query_as::<_, data::App>("SELECT * FROM apps WHERE app_name = ? COLLATE NOCASE")
        .bind(app.to_lowercase())
        .fetch_one(&db)
        .await
        .wrap_err(format!("Failed to find app named '{}'", app))
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
        .wrap_err(format!(
            "Failed to update last opened for app with id '{}'",
            id
        ))
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
