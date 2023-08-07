-- Your SQL goes here
CREATE TABLE payments (
    payment_id UUID PRIMARY KEY NOT NULL,
    price NUMERIC(10, 2) NOT NULL,
    user_id UUID NOT NULL,
    order_id UUID NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users (user_id),
    FOREIGN KEY(order_id) REFERENCES ad_orders (order_id)
);