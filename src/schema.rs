// @generated automatically by Diesel CLI.

diesel::table! {
    ad_categories (category_id, ad_id) {
        category_id -> Uuid,
        ad_id -> Uuid,
    }
}

diesel::table! {
    ad_orders (ad_order_id) {
        ad_order_id -> Uuid,
        start_time -> Timestamptz,
        end_time -> Timestamptz,
        price -> Float8,
        is_rejected -> Bool,
        ad_id -> Uuid,
        screen_id -> Uuid,
    }
}

diesel::table! {
    addresses (address_id) {
        address_id -> Uuid,
        address_name -> Text,
        business_id -> Uuid,
    }
}

diesel::table! {
    admin (admin_id) {
        admin_id -> Uuid,
        admin_name -> Text,
        password -> Text,
    }
}

diesel::table! {
    ads (ad_id) {
        ad_id -> Uuid,
        ad_name -> Text,
        img_url -> Text,
        status -> Text,
        user_id -> Uuid,
    }
}

diesel::table! {
    business_categories (category_id, business_id) {
        category_id -> Uuid,
        business_id -> Uuid,
    }
}

diesel::table! {
    businesses (business_id) {
        business_id -> Uuid,
        business_name -> Text,
        email -> Text,
        password -> Text,
        phone_number -> Text,
        img_url -> Text,
    }
}

diesel::table! {
    categories (category_id) {
        category_id -> Uuid,
        category_name -> Text,
    }
}

diesel::table! {
    incomes (income_id) {
        income_id -> Uuid,
        income -> Float8,
        business_id -> Uuid,
        ad_order_id -> Uuid,
    }
}

diesel::table! {
    payments (payment_id) {
        payment_id -> Uuid,
        price -> Float8,
        user_id -> Uuid,
        ad_order_id -> Uuid,
    }
}

diesel::table! {
    screens (screen_id) {
        screen_id -> Uuid,
        screen_name -> Text,
        price_per_time -> Float8,
        characteristics -> Text,
        traffic -> Int4,
        business_id -> Uuid,
        address_id -> Uuid,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        user_name -> Text,
        img_url -> Text,
        email -> Text,
        password -> Text,
        phone_number -> Text,
    }
}

diesel::joinable!(ad_categories -> ads (ad_id));
diesel::joinable!(ad_categories -> categories (category_id));
diesel::joinable!(ad_orders -> ads (ad_id));
diesel::joinable!(ad_orders -> screens (screen_id));
diesel::joinable!(addresses -> businesses (business_id));
diesel::joinable!(ads -> users (user_id));
diesel::joinable!(business_categories -> businesses (business_id));
diesel::joinable!(business_categories -> categories (category_id));
diesel::joinable!(incomes -> ad_orders (ad_order_id));
diesel::joinable!(incomes -> businesses (business_id));
diesel::joinable!(payments -> ad_orders (ad_order_id));
diesel::joinable!(payments -> users (user_id));
diesel::joinable!(screens -> addresses (address_id));
diesel::joinable!(screens -> businesses (business_id));

diesel::allow_tables_to_appear_in_same_query!(
    ad_categories,
    ad_orders,
    addresses,
    admin,
    ads,
    business_categories,
    businesses,
    categories,
    incomes,
    payments,
    screens,
    users,
);
