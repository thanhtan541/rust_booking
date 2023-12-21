use diesel::prelude::*;
use chrono::prelude::*;

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::infrastructure::database::schema::bookings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Booking {
    pub reference: String,
    pub start_date: String,
    pub end_date: String,
}

impl Booking {
    pub fn new(reference: String, start_date: String, end_date: String) -> Self {
        Self {
            reference,
            start_date,
            end_date,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BookingSnapshot {
    pub reference: String,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub total: i32,
}

impl BookingSnapshot {
    pub fn new(reference: String, from: DateTime<Utc>, to: DateTime<Utc>, total: i32) -> Self {
        Self {
            reference,
            from,
            to,
            total,
        }
    }

    pub fn duration(&self) -> i32 {
        self.to.signed_duration_since(self.from).num_days() as i32
    }

    pub fn total(&self) -> i32 {
        self.total
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_raw_booking() {
        let booking = BookingSnapshot::new(
            String::from("raw_booking"),
            Utc.with_ymd_and_hms(2023, 12, 17, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2023, 12, 20, 0, 0, 0).unwrap(),
            1,
        );

        assert_eq!(3, booking.duration());
        assert_eq!(1, booking.total());
    }
}

