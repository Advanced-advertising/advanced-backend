-- Your SQL goes here
CREATE TABLE users (
    user_id UUID PRIMARY KEY NOT NULL,
    user_name TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL
);