CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS service_users CASCADE;

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    phone_number TEXT NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    created timestamptz DEFAULT now(),
    last_login timestamptz NULL
);

CREATE TABLE service_users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username TEXT NOT NULL,
    passphrase TEXT NOT NULL,
    created timestamptz DEFAULT now(),
    last_login timestamptz NULL
);
