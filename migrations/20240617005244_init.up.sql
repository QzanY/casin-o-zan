-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS users(
        name TEXT,
        password TEXT,
        balance INTEGER,
        items JSONB DEFAULT '[]'::JSONB
    );
CREATE TABLE
    IF NOT EXISTS prices(
        name TEXT,
        price INTEGER
    );