CREATE TABLE IF NOT EXISTS apps
(
    id                      INTEGER PRIMARY KEY NOT NULL,
    app_name                VARCHAR(250) NOT NULL UNIQUE,
    exe_name                VARCHAR(250) NOT NULL,
    search_term             VARCHAR(250) NOT NULL,
    search_method           VARCHAR(250) NOT NULL,
    app_path                VARCHAR(250) NULL,
    last_opened             DATETIME NULL,
    last_updated            DATETIME NULL
);