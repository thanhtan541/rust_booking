use crate::prelude::*;

use diesel::prelude::*;

use crate::domain::booking::booking::Booking;
use crate::domain::booking::booking_repository::BookingRepository;
use crate::infrastructure::database::schema::bookings::dsl::*;

pub struct BookingRepositoryImpl<'a> {
    conn: &'a mut PgConnection,
    entity: bookings,
}

impl BookingRepositoryImpl<'_> {
    pub fn new(conn: &mut PgConnection) -> Self {
        Self {
            conn,
            entity: bookings,
        }
    }
}

impl BookingRepository for BookingRepositoryImpl<'_> {
    fn find_by_ref(&self, ref_no: String) -> Result<Booking> {
        let results = self
            .entity
            .filter(reference.eq(ref_no))
            .limit(1)
            .select(Booking::as_select())
            .load(self.conn)
            .expect("Error loading posts");

        return Ok(results[0]);
    }

    fn save(&mut self, booking: Booking) -> Result<bool> {
        Ok(true)
    }
}
