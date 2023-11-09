use crate::domain::booking::entity::Booking;

pub trait BookingService {
    fn register(&mut self) -> Result<Booking, String>;
    fn get_booking(&self, reference: u32) -> Option<Booking>;
}
