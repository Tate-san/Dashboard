-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS roles(
        role_id SERIAL PRIMARY KEY NOT NULL,
        role_name VARCHAR(255) NOT NULL UNIQUE
    );