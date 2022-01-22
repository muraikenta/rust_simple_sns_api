table! {
    users (id) {
        id -> Integer,
        email -> Varchar,
        password -> Varchar,
        name -> Varchar,
        icon_url -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}
