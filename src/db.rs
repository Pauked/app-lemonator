use crate::data;
use chrono::Utc;
use eyre::{Context, Report};
use log::debug;
use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    Sqlite, SqlitePool,
};
use std::{
    fs::{self},
    io,
};

const DB_URL: &str = "sqlite://app-lemonator.db";
const DB_FILE: &str = "app-lemonator.db";

static MIGRATOR: Migrator = sqlx::migrate!(); // this will pick up migrations from the ./migrations directory

async fn get_db() -> sqlx::Pool<Sqlite> {
    SqlitePool::connect(DB_URL).await.unwrap()
}

pub fn database_exists() -> bool {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async { Sqlite::database_exists(DB_URL).await.unwrap_or(false) })
}

pub fn reset_db() -> Result<(), io::Error> {
    fs::remove_file(DB_FILE)
}

pub fn create_db() -> Result<bool, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            debug!("Creating database {}", DB_URL);
            Sqlite::create_database(DB_URL)
                .await
                .wrap_err("Unable to create database")?;
            debug!("Successfully created database");
        } else {
            debug!("Database already exists");
        }

        let db = get_db().await;
        MIGRATOR
            .run(&db)
            .await
            .wrap_err("Unable to run database migrations")?;
        debug!("Migration success");
        Ok(true)
    })
}

pub fn add_app(app: &data::App) -> Result<sqlx::sqlite::SqliteQueryResult, Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
    let db = get_db().await;

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
})
}

pub fn edit_app(
    lookup_app_name: &str,
    app: &data::App,
) -> Result<sqlx::sqlite::SqliteQueryResult, Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
    let db = get_db().await;

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
})
}

pub fn get_app(app: &str) -> Result<data::App, Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::App>("SELECT * FROM apps WHERE app_name = ? COLLATE NOCASE")
            .bind(app.to_lowercase())
            .fetch_one(&db)
            .await
            .wrap_err(format!("Failed to find app named '{}'", app))
    })
}

pub fn get_apps() -> Result<Vec<data::App>, Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::App>("SELECT * FROM apps")
            .fetch_all(&db)
            .await
            .wrap_err("Failed to get list of all apps")
    })
}

pub fn update_app_file_version(
    id: i32,
    app_file_version: &data::FileVersion,
) -> Result<sqlx::sqlite::SqliteQueryResult, Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query(
            "UPDATE apps SET app_path = $2, app_description = $3, app_version = $4,
     last_updated = $5 WHERE id=$1 COLLATE NOCASE",
        )
        .bind(id)
        .bind(app_file_version.path.clone())
        .bind(app_file_version.app_description.clone())
        .bind(app_file_version.display_version())
        .bind(Utc::now())
        .execute(&db)
        .await
        .wrap_err(format!(
            "Failed to update app path '{}' for app with id '{}'",
            app_file_version.path, id
        ))
    })
}

pub fn update_last_opened(id: i32) -> Result<sqlx::sqlite::SqliteQueryResult, Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("UPDATE apps SET last_opened = $1 WHERE id=$2 COLLATE NOCASE")
            .bind(Utc::now())
            .bind(id)
            .execute(&db)
            .await
            .wrap_err(format!(
                "Failed to update last opened for app with id '{}'",
                id
            ))
    })
}

pub fn delete_app(app: &str) -> Result<sqlx::sqlite::SqliteQueryResult, Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("DELETE FROM apps WHERE app_name=$1 COLLATE NOCASE")
            .bind(app.to_lowercase())
            .execute(&db)
            .await
            .wrap_err(format!("Failed to delete app '{}'", app))
    })
}
