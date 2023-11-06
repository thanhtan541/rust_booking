use crate::domain::booking::booking::Booking;
use crate::prelude::*;
use std::collections::HashMap;

pub trait BookingRepository {
    fn find_by_ref(&mut self, ref_no: String) -> Result<Booking>;
    fn save(&mut self, user: Booking) -> Result<bool>;
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
