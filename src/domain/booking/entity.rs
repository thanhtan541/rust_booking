use diesel::prelude::*;

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

pub struct Room {
    pub reference: String,
    pub name: String,
    pub description: String,
}

pub struct RoomEvent {
    pub reference: String,
    pub start_date: String,
    pub end_date: String,
    pub r#type: String,
    pub room_ref: String,
}

#[cfg(test)]
mod test {
    use crate::domain::booking::entity::Booking;

    #[test]
    fn create_client() {
        let reference = "123".to_string();
        let start_date = "123".to_string();
        let end_date = "123".to_string();

        let booking = Booking::new(reference, start_date, end_date);

        assert_eq!(booking.reference, "123");
        assert_eq!(booking.start_date, "123");
        assert_eq!(booking.end_date, "123");
    }
}

