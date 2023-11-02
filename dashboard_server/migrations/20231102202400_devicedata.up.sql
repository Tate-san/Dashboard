-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS devicedata(
        devicedata_id SERIAL PRIMARY KEY NOT NULL,
        device_id INTEGER NOT NULL,
        name VARCHAR(255) NOT NULL,
        value VARCHAR(255) NOT NULL
    );