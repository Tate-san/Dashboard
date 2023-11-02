
ALTER TABLE systems
    DROP CONSTRAINT fk_owner_id;


ALTER TABLE systemaccess
    DROP CONSTRAINT fk_user_id;

ALTER TABLE systemaccess
    DROP CONSTRAINT fk_system_id;

ALTER TABLE devicestructure 
    DROP CONSTRAINT fk_device_id;

ALTER TABLE devicedata 
    DROP CONSTRAINT fk_device_id;