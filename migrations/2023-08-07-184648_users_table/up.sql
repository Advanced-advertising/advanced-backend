-- Your SQL goes here
CREATE TABLE users (
    user_id UUID PRIMARY KEY NOT NULL,
    user_name TEXT NOT NULL,
    img_url TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    phone_number TEXT NOT NULL
);