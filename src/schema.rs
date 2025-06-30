// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        nom -> Varchar,
        balance -> Nullable<Int4>,
        myca_id -> Nullable<Int4>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
    }
}
