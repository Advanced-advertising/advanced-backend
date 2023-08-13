-- Your SQL goes here
CREATE TABLE screens (
    screen_id Uuid PRIMARY KEY NOT NULL,
    screen_name TEXT NOT NULL,
    price_per_time double precision NOT NULL,
    characteristics TEXT NOT NULL,
    traffic INTEGER NOT NULL,
    business_id Uuid NOT NULL,
    address_id Uuid NOT NULL,
    FOREIGN KEY(business_id) REFERENCES businesses (business_id),
    FOREIGN KEY(address_id) REFERENCES addresses (address_id)
);