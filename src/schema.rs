// @generated automatically by Diesel CLI.

diesel::table! {
    clients (uuid) {
        #[max_length = 36]
        uuid -> Char,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        archived -> Nullable<Bool>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(clients,);
