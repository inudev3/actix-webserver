// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        stock -> Float8,
        price -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 100]
        company -> Varchar,
        #[max_length = 64]
        password -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    products,
    users,
);
