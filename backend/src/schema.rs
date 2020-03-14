table! {
    projects (id) {
        id -> Int4,
        user_id -> Int4,
        subject -> Varchar,
        website -> Varchar,
        source_code -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        username -> Varchar,
        password -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        banned -> Bool,
    }
}

joinable!(projects -> users (user_id));

allow_tables_to_appear_in_same_query!(
    projects,
    users,
);
