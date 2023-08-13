-- Your SQL goes here
CREATE TABLE incomes (
    income_id UUID PRIMARY KEY NOT NULL,
    income double precision NOT NULL,
    business_id UUID NOT NULL,
    order_id UUID NOT NULL,

    FOREIGN KEY(business_id) REFERENCES businesses (business_id),
    FOREIGN KEY(order_id) REFERENCES ad_orders (order_id)
);