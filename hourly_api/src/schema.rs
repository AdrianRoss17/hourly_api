// @generated automatically by Diesel CLI.

diesel::table! {
    bird (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        scientific_name -> Varchar,
        #[max_length = 255]
        commonwealth_status -> Varchar,
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
    bird,
    users,
);
