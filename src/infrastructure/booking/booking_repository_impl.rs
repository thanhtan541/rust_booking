use crate::prelude::*;

use diesel::prelude::*;

use crate::domain::booking::entity::Booking;
use crate::domain::booking::repository::BookingRepository;
use crate::infrastructure::database::schema::bookings::dsl::*;

pub struct BookingRepositoryImpl<'a> {
    conn: &'a mut PgConnection,
    entity: bookings,
}

impl<'a> BookingRepositoryImpl<'a> {
    pub fn new(conn: &'a mut PgConnection) -> BookingRepositoryImpl<'a> {
        BookingRepositoryImpl {
            conn,
            entity: bookings,
        }
    }
}

impl BookingRepository for BookingRepositoryImpl<'_> {
    fn find_by_ref(&mut self, ref_no: String) -> Result<Booking> {
        let result = self
            .entity
            .filter(reference.eq(ref_no))
            .limit(1)
            .select(Booking::as_select())
            .first(self.conn)
            .expect("Error loading posts");

        Ok(result)
    }

    fn save(&mut self, _booking: Booking) -> Result<bool> {
        Ok(true)
    }
}
