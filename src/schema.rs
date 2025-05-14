// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        nom -> Varchar,
        balance -> Nullable<Int4>,
    }
}
