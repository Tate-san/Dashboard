-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS devicedata(
        devicedata_id SERIAL PRIMARY KEY NOT NULL,
        device_id INTEGER NOT NULL,
        devicestructure_id INTEGER NOT NULL,
        value VARCHAR(255) NOT NULL,
        timestamp TIMESTAMPTZ NOT NULL
    );