

diesel::table! {
    password (user_id) {
        user_id -> Integer,
        #[max_length = 255]
        passwords -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    password,
    users
);
