CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create tables
CREATE TABLE users(
    id uuid DEFAULT uuid_generate_v4(),
    username VARCHAR(24) NOT NULL,
    first_name VARCHAR(80) NOT NULL,
    last_name VARCHAR(80) NOT NULL,
    password_hash BYTEA NOT NULL,
    assigned_role SMALLINT NOT NULL CHECK(assigned_role IN (1, 2, 3)),

    PRIMARY KEY (id),
    UNIQUE(username)
);

CREATE TABLE sessions(
    id uuid DEFAULT uuid_generate_v4(),
    user_id uuid NOT NULL REFERENCES users(id),
    created_time TIMESTAMP NOT NULL,
    last_login TIMESTAMP NOT NULL,
    logout_time TIMESTAMP,
    device_os SMALLINT NOT NULL CHECK(device_os IN (1, 2, 3)),
    device_name VARCHAR(64) NOT NULL,
    device_hash BYTEA NOT NULL,

    PRIMARY KEY(id),
    UNIQUE(device_hash)
);

CREATE TABLE area(
    code VARCHAR(10) NOT NULL,
    name VARCHAR(128) NOT NULL,

    PRIMARY KEY(code)
);

CREATE TABLE protocol_violators(
    id uuid DEFAULT uuid_generate_v4(),
    first_name VARCHAR(48) NOT NULL,
    last_name VARCHAR(48) NOT NULL,
    category VARCHAR(8) NOT NULL CHECK(category IN ('student', 'visitor', 'faculty', 'staff')),

    PRIMARY KEY(id)
);

CREATE TABLE protocol_violations(
    id uuid DEFAULT uuid_generate_v4(),
    personnel_id uuid NOT NULL REFERENCES users(id),
    date_time TIMESTAMP NOT NULL,
    area_code VARCHAR(10) NOT NULL REFERENCES area(code),
    category SMALLINT NOT NULL CHECK(category IN (1, 2)),

    PRIMARY KEY(id)
);

-- Sample users
DO $$
    DECLARE password_argon2 BYTEA := decode('Wo+1JdceNVhCjdKSAPq6bDUJJKgFfwhJyCPjCXVQ6tp7lAeHu6YMsmJB3AYusuLsO9ym6EemeN2FJnWOx/Ta2g==', 'base64');
BEGIN
    INSERT INTO users(username, first_name, last_name, password_hash, assigned_role)
    VALUES
    ('vladimir', 'Vlad', 'Tepes', password_argon2, 1),
    ('rio', 'Rio', 'LeBlanc', password_argon2, 2),
    ('admin', 'Lien', 'Walker', password_argon2, 3);
END $$;

-- Configure privileges
GRANT
    SELECT, INSERT, UPDATE 
    ON users TO unc_client;

GRANT
    SELECT, INSERT, UPDATE 
    ON sessions TO unc_client;

GRANT
    SELECT, INSERT, UPDATE 
    ON area TO unc_client;

GRANT
    SELECT, INSERT, UPDATE 
    ON protocol_violators TO unc_client;

GRANT
    SELECT, INSERT, UPDATE 
    ON protocol_violations TO unc_client;