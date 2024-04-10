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
        #[max_length = 255]
        first_name -> Nullable<Varchar>,
        #[max_length = 255]
        last_name -> Nullable<Varchar>,
        #[max_length = 255]
        id -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bird,
    users,
);
