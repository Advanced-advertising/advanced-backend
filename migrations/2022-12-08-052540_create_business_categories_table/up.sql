-- Your SQL goes here
CREATE TABLE business_categories (
    category_id UUID NOT NULL,
    business_id UUID NOT NULL,

    PRIMARY KEY (category_id, business_id),
    FOREIGN KEY(category_id) REFERENCES categories (category_id),
    FOREIGN KEY(business_id) REFERENCES businesses (business_id)
);