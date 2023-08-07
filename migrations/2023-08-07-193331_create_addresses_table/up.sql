-- Your SQL goes here
CREATE TABLE addresses (
    address_id Uuid PRIMARY KEY NOT NULL,
    address_name TEXT NOT NULL,
    business_id UUID REFERENCES public.businesses(business_id)
);