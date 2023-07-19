CREATE TABLE IF NOT EXISTS apps
(
    id              INTEGER PRIMARY KEY NOT NULL,
    app             VARCHAR(250) NOT NULL UNIQUE,
    exe_name        VARCHAR(250) NOT NULL,
    search_term     VARCHAR(250) NOT NULL,
    search_method   VARCHAR(250) NOT NULL
);