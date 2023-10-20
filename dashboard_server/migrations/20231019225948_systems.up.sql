-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS systems(
        system_id SERIAL PRIMARY KEY NOT NULL,
        name VARCHAR(255) NOT NULL,
        description VARCHAR(255) NOT NULL,
        owner_id INTEGER NOT NULL
    );