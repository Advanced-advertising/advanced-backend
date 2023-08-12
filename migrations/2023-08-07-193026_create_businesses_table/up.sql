-- Your SQL goes here
CREATE TABLE businesses (
    business_id Uuid PRIMARY KEY NOT NULL,
    business_name TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    img_url TEXT NOT NULL
);
