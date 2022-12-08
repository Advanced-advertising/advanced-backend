-- Your SQL goes here
CREATE TABLE addresses (
    address_id UUID PRIMARY KEY NOT NULL,
    address_name TEXT NOT NULL,
    business_id UUID NOT NULL,
    FOREIGN KEY(business_id) REFERENCES businesses (business_id)
);