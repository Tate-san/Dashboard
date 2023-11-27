-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS systemdevices(
        systemdevices_id SERIAL PRIMARY KEY NOT NULL,
        system_id INTEGER NOT NULL,
        device_id INTEGER NOT NULL
    );

ALTER TABLE systemdevices 
    ADD CONSTRAINT fk_device_id
        FOREIGN KEY(device_id) 
        REFERENCES devices(device_id)
        ON DELETE CASCADE;

ALTER TABLE systemdevices 
    ADD CONSTRAINT fk_system_id
        FOREIGN KEY(system_id) 
        REFERENCES systems(system_id)
        ON DELETE CASCADE;