use crate::domain::booking::booking::Booking;
use crate::domain::booking::booking_repository::{BookingRepository, InMemoryBookingRepository};

impl BookingRepository for InMemoryBookingRepository {
    fn find_by_ref(&self, id: u32) -> Option<Booking> {
        self.storage.get(&id).cloned()
    }

    fn save(&mut self, booking: Booking) {
        self.storage.insert(booking.reference, booking);
    }
}
