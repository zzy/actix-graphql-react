table! {
    sites (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        password -> Varchar,
        bio -> Nullable<Text>,
        avatar -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(sites, users,);
