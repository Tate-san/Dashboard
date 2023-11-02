-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS devicestructure(
        devicestructure_id SERIAL PRIMARY KEY NOT NULL,
        device_id INTEGER NOT NULL,
        real_name VARCHAR(255) NOT NULL,
        alias_name VARCHAR(255) NOT NULL,
        type VARCHAR(255) NOT NULL
    );