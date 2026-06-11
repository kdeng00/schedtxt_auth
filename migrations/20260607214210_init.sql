-- Add migration script here
CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS "user" (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    firstname TEXT NOT NULL,
    lastname TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    salt_id UUID NOT NULL
);


CREATE TABLE IF NOT EXISTS "salt" (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    salt TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS "service_user" (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username TEXT NOT NULL,
    passphrase TEXT NOT NULL,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login timestamptz NULL,
    salt_id UUID NOT NULL
);
