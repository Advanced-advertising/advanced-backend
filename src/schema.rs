table! {
    businesses (business_id) {
        business_id -> Uuid,
        business_name -> Text,
        email -> Text,
        password -> Text,
        working_time -> Text,
        img_url -> Text,
    }
}

table! {
    screens (screen_id) {
        screen_id -> Uuid,
        screen_name -> Text,
        price_per_time -> Text,
        characteristics -> Text,
        business_id -> Uuid,
    }
}

table! {
    users (user_id) {
        user_id -> Uuid,
        user_name -> Text,
        email -> Text,
        password -> Text,
    }
}

joinable!(screens -> businesses (business_id));

allow_tables_to_appear_in_same_query!(
    businesses,
    screens,
    users,
);
