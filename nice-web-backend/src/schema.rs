// @generated automatically by Diesel CLI.

diesel::table! {
    post (post_id) {
        post_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        user_id -> Int4,
        bookmarked -> Bool,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 128]
        password_hash -> Varchar,
    }
}

diesel::joinable!(post -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(post, users,);
