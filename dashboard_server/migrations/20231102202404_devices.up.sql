-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS devices(
        device_id SERIAL PRIMARY KEY NOT NULL,
        owner_id INTEGER NOT NULL,
        name VARCHAR(255) NOT NULL UNIQUE,
        topic VARCHAR(255) NOT NULL
    );