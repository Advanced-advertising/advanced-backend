-- Your SQL goes here
CREATE TABLE ad_orders (
    order_id UUID PRIMARY KEY NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    price TEXT NOT NULL,
    is_rejected BOOL NOT NULL,
    ad_id UUID NOT NULL,
    screen_id UUID NOT NULL,

    FOREIGN KEY(ad_id) REFERENCES ads (ad_id),
    FOREIGN KEY(screen_id) REFERENCES screens (screen_id)
);