// @generated automatically by Diesel CLI.

diesel::table! {
    bookings (id) {
        id -> Int4,
        start_date -> Varchar,
        end_date -> Varchar,
    }
}

