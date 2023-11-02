ALTER TABLE systems
    ADD CONSTRAINT fk_owner_id
        FOREIGN KEY(owner_id) 
        REFERENCES users(user_id);


ALTER TABLE systemaccess
    ADD CONSTRAINT fk_user_id
        FOREIGN KEY(user_id) 
        REFERENCES users(user_id);

ALTER TABLE systemaccess
    ADD CONSTRAINT fk_system_id
        FOREIGN KEY(system_id) 
        REFERENCES systems(system_id);

ALTER TABLE devicestructure 
    ADD CONSTRAINT fk_device_id
        FOREIGN KEY(device_id) 
        REFERENCES devices(device_id);

ALTER TABLE devicedata 
    ADD CONSTRAINT fk_device_id
        FOREIGN KEY(device_id) 
        REFERENCES devices(device_id);