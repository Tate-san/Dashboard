-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS users(
        user_id SERIAL PRIMARY KEY NOT NULL,
        username VARCHAR(255) NOT NULL UNIQUE,
        password VARCHAR(255) NOT NULL,
        role INTEGER 
    );