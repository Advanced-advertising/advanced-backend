-- Your SQL goes here
CREATE TABLE screens (
    screen_id Uuid PRIMARY KEY NOT NULL,
    screen_name TEXT NOT NULL,
    price_per_time TEXT NOT NULL,
    characteristics TEXT NOT NULL,
    business_id Uuid NOT NULL,
    FOREIGN KEY(business_id) REFERENCES businesses (business_id)
);