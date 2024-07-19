// @generated automatically by Diesel CLI.

diesel::table! {
    alembic_version (version_num) {
        #[max_length = 32]
        version_num -> Varchar,
    }
}

diesel::table! {
    role (id) {
        id -> Int4,
        name -> Varchar,
        permissions -> Nullable<Json>,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 320]
        email -> Varchar,
        #[max_length = 50]
        username -> Varchar,
        registered_at -> Nullable<Timestamp>,
        role_id -> Int4,
        hashed_password -> Varchar,
        is_active -> Bool,
        is_superuser -> Bool,
        is_verified -> Bool,
        #[max_length = 1024]
        storage -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        surname -> Varchar,
        age -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        hash_password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        #[max_length = 50]
        role -> Varchar,
        #[max_length = 255]
        avatar -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
        #[max_length = 255]
        token -> Nullable<Varchar>,
        followers -> Nullable<Array<Nullable<Int4>>>,
        followings -> Nullable<Array<Nullable<Int4>>>,
        bio -> Nullable<Text>,
        favorite_genres -> Nullable<Array<Nullable<Text>>>,
        last_active -> Timestamp,
        recently_played -> Nullable<Array<Nullable<Text>>>,
        liked_songs -> Nullable<Array<Nullable<Text>>>,
        social_links -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::joinable!(user -> role (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    alembic_version,
    role,
    user,
    users,
);
