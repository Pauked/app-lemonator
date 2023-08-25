CREATE TABLE IF NOT EXISTS apps
(
    id                      INTEGER PRIMARY KEY NOT NULL,
    app_name                TEXT NOT NULL UNIQUE,
    exe_name                TEXT NOT NULL,
    params                  TEXT NULL,
    search_term             TEXT NOT NULL,
    search_method           TEXT NOT NULL,
    app_path                TEXT NULL,
    last_opened             DATETIME NULL,
    last_updated            DATETIME NULL,
    operating_system        TEXT NOT NULL
);