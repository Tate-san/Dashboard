
ALTER TABLE users
    DROP CONSTRAINT fk_role_id;


ALTER TABLE systems
    DROP CONSTRAINT fk_owner_id;


ALTER TABLE systemaccess
    DROP CONSTRAINT fk_user_id;

ALTER TABLE systemaccess
    DROP CONSTRAINT fk_system_id;