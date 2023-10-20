
ALTER TABLE users
    ADD CONSTRAINT fk_role_id
        FOREIGN KEY(role_id) 
        REFERENCES roles(role_id);


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