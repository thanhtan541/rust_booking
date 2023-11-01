diesel::table! {
  bookings (id) {
      id -> Int4,
      reference -> Varchar,
      start_date -> Varchar,
      end_date -> Varchar,
  }
}
