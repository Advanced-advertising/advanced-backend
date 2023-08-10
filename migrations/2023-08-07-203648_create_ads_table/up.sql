-- Your SQL goes here
CREATE TABLE ads (
                     ad_id UUID PRIMARY KEY NOT NULL,
                     ad_name TEXT NOT NULL,
                     img_url TEXT NOT NULL,
                     status TEXT NOT NULL,
                     user_id Uuid NOT NULL,
                     FOREIGN KEY(user_id) REFERENCES users (user_id)
);