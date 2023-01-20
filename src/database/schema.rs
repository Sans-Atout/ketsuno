// @generated automatically by Diesel CLI.

diesel::table! {
    api_roles (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    users_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

diesel::table! {
    web_user (id) {
        id -> Int4,
        pseudo -> Varchar,
        email -> Varchar,
        avatar -> Varchar,
        password -> Varchar,
        is_activated -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    api_roles,
    users_roles,
    web_user,
);
