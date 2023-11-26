ALTER TABLE systems
    ADD CONSTRAINT fk_owner_id
        FOREIGN KEY(owner_id) 
        REFERENCES users(user_id)
        ON DELETE CASCADE;

ALTER TABLE systemaccess
    ADD CONSTRAINT fk_user_id
        FOREIGN KEY(user_id) 
        REFERENCES users(user_id);

ALTER TABLE systemaccess
    ADD CONSTRAINT fk_system_id
        FOREIGN KEY(system_id) 
        REFERENCES systems(system_id)
        ON DELETE CASCADE;

ALTER TABLE devicestructure 
    ADD CONSTRAINT fk_device_id
        FOREIGN KEY(device_id) 
        REFERENCES devices(device_id)
        ON DELETE CASCADE;

ALTER TABLE devicedata 
    ADD CONSTRAINT fk_device_id
        FOREIGN KEY(device_id) 
        REFERENCES devices(device_id)
        ON DELETE CASCADE;

ALTER TABLE devicedata 
    ADD CONSTRAINT fk_devicestructure_id
        FOREIGN KEY(devicestructure_id) 
        REFERENCES devicestructure(devicestructure_id)
        ON DELETE CASCADE;

ALTER TABLE devices 
    ADD CONSTRAINT fk_user_id
        FOREIGN KEY(owner_id) 
        REFERENCES users(user_id)
        ON DELETE CASCADE;