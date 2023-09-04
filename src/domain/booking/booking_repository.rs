use crate::domain::booking::booking::Booking;
use std::collections::HashMap;

pub trait BookingRepository {
    fn find_by_ref(&self, id: u32) -> Option<Booking>;
    fn save(&mut self, user: Booking);
}

pub struct InMemoryBookingRepository {
    pub storage: HashMap<u32, Booking>,
}

impl InMemoryBookingRepository {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }
}
