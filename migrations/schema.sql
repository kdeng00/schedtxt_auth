CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS service_users CASCADE;

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    first_name TEXT NULL,
    last_name TEXT NULL,
    phone_number TEXT NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    created timestamptz DEFAULT now(),
    last_login timestamptz NULL,
    salt_id UUID NOT NULL
);

CREATE TABLE IF NOT EXISTS "salt" (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    salt TEXT NOT NULL
);

CREATE TABLE service_users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username TEXT NOT NULL,
    passphrase TEXT NOT NULL,
    created timestamptz DEFAULT now(),
    last_login timestamptz NULL
);
