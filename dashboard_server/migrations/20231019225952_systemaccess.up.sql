-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS systemaccess(
        systemaccess_id SERIAL PRIMARY KEY NOT NULL,
        user_id INTEGER NOT NULL,
        system_id INTEGER NOT NULL
    );