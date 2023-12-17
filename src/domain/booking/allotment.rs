use chrono::prelude::*;

pub struct Room {
    pub reference: String,
}

impl Room {
    pub fn new(reference: String) -> Self {
        Self {
            reference,
        }
    }
}

pub struct RoomEvent {
    pub reference: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

impl RoomEvent {
    pub fn new(reference: String, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Self {
        Self {
            reference,
            start_date,
            end_date,
        }
    }
}


#[cfg(test)]
mod test {
    use crate::domain::booking::allotment::*;

    #[test]
    fn create_rooms() {
        // create rooms
        let mut rooms = Vec::new();
        rooms.push(Room::new("1".to_string()));
        rooms.push(Room::new("2".to_string()));
        rooms.push(Room::new("3".to_string()));

        assert_eq!(rooms.len(), 3);

        let mut rooms_events = Vec::new();
        rooms_events.push(RoomEvent::new(
            "1".to_string(),
            Utc.with_ymd_and_hms(2023, 12, 17, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2023, 12, 18, 0, 0, 0).unwrap(),
        ));
        rooms_events.push(RoomEvent::new(
            "1".to_string(),
            Utc.with_ymd_and_hms(2023, 12, 17, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2023, 12, 18, 0, 0, 0).unwrap(),
        ));
        rooms_events.push(RoomEvent::new(
            "1".to_string(),
            Utc.with_ymd_and_hms(2023, 12, 17, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2023, 12, 18, 0, 0, 0).unwrap(),
        ));

        assert_eq!(rooms_events.len(), 3);
    }
}

