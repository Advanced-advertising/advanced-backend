-- Your SQL goes here
CREATE TABLE ad_orders (
                           order_id UUID PRIMARY KEY NOT NULL,
                           start_time TIMESTAMPTZ NOT NULL,
                           end_time TIMESTAMPTZ NOT NULL,
                           price NUMERIC(2, 10) NOT NULL,
                           is_rejected BOOL NOT NULL,
                           ad_id UUID NOT NULL,
                           screen_id UUID NOT NULL,

                           FOREIGN KEY(ad_id) REFERENCES ads (ad_id),
                           FOREIGN KEY(screen_id) REFERENCES screens (screen_id)
);