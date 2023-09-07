// @generated automatically by Diesel CLI.

diesel::table! {
    important_posts (user_id, post_id) {
        user_id -> Uuid,
        post_id -> Uuid,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
        user_id -> Uuid,
        title -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 256]
        password -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(important_posts -> posts (post_id));
diesel::joinable!(important_posts -> users (user_id));
diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(important_posts, posts, users,);
