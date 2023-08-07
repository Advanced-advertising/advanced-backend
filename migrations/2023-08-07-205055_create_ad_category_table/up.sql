-- Your SQL goes here
CREATE TABLE ad_categories (
    category_id UUID NOT NULL,
    ad_id UUID NOT NULL,

    PRIMARY KEY (category_id, ad_id),
    FOREIGN KEY(category_id) REFERENCES categories (category_id),
    FOREIGN KEY(ad_id) REFERENCES ads (ad_id)
);