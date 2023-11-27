-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS kpi(
        kpi_id SERIAL PRIMARY KEY NOT NULL,
        parameter VARCHAR(255) NOT NULL,
        limitation VARCHAR(255) NOT NULL,
        value VARCHAR(255) NOT NULL,
        owner_id INTEGER NOT NULL
    );

ALTER TABLE kpi 
    ADD CONSTRAINT fk_owner_id
        FOREIGN KEY(owner_id) 
        REFERENCES users(user_id)
        ON DELETE CASCADE;