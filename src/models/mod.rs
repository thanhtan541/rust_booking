use super::schema::bookings;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = bookings)]
pub struct Booking {
    pub id: i32,
    pub start_date: String,
    pub end_date: String,
}
