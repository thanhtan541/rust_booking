pub struct Booking {
    pub reference: String,
    pub start_date: String,
    pub end_date: String,
}

impl Booking {
    pub fn new(reference: &str, start_date: &str, end_date: &str) -> Self {
        Self {
            reference: reference.to_string(),
            start_date: start_date.to_string(),
            end_date: end_date.to_string(),
        }
    }

    pub fn edit(&mut self, start_date: &str, end_date: &str) {
        self.start_date = start_date.to_string();
        self.end_date = end_date.to_string();
    }
}

#[cfg(test)]
mod test {
    use crate::domain::booking::Booking;

    #[test]
    fn create_client() {
        let reference: String = "123".to_string();
        let start_date: String = "123".to_string();
        let end_date: String = "123".to_string();

        let booking = Booking::new(reference.as_str(), start_date.as_str(), end_date.as_str());

        assert_eq!(booking.reference, reference.as_str());
        assert_eq!(booking.start_date, start_date.as_str());
        assert_eq!(booking.end_date, end_date.as_str());
    }
}
