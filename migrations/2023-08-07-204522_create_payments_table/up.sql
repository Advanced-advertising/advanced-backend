-- Your SQL goes here
CREATE TABLE payments (
    payment_id UUID PRIMARY KEY NOT NULL,
    price double precision NOT NULL,
    user_id UUID NOT NULL,
    ad_order_id UUID NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users (user_id),
    FOREIGN KEY(ad_order_id) REFERENCES ad_orders (ad_order_id)
);