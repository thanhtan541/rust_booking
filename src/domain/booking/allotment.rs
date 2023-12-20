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
    pub room_id: i32,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

impl RoomEvent {
    pub fn new(reference: String, room_id: i32, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Self {
        Self {
            reference,
            room_id,
            start_date,
            end_date,
        }
    }
}

fn find_occupied_rooms(room_events: Vec<RoomEvent>, from: DateTime<Utc>, to: DateTime<Utc>) -> Vec<i32> {
    let mut occupied_rooms: Vec<i32> = Vec::new();
    for room_event in room_events {
        // Event that has end date less than or equal to checkin date should not affect room
        // availability
        if room_event.end_date <= from || room_event.start_date >= to {
            continue;
        }

        if room_event.end_date > from || room_event.end_date < to {
            occupied_rooms.push(room_event.room_id);
            continue;
        }

        if room_event.start_date > from || room_event.start_date < to {
            occupied_rooms.push(room_event.room_id);
            continue;
        }
    }

    return occupied_rooms;

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_busy_rooms() {
        // create rooms
        let mut rooms = Vec::new();
        rooms.push(Room::new(String::from("1")));
        rooms.push(Room::new(String::from("2")));
        rooms.push(Room::new(String::from("3")));

        assert_eq!(rooms.len(), 3);

        let mut rooms_events = Vec::new();
        rooms_events.push(RoomEvent::new(
            "1".to_string(),
            1,
            Utc.with_ymd_and_hms(2023, 12, 17, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2023, 12, 20, 0, 0, 0).unwrap(),
        ));
        rooms_events.push(RoomEvent::new(
            "2".to_string(),
            2,
            Utc.with_ymd_and_hms(2023, 12, 17, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2023, 12, 20, 0, 0, 0).unwrap(),
        ));
        rooms_events.push(RoomEvent::new(
            "3".to_string(),
            3,
            Utc.with_ymd_and_hms(2023, 12, 12, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2023, 12, 17, 0, 0, 0).unwrap(),
        ));
        rooms_events.push(RoomEvent::new(
            "3".to_string(),
            3,
            Utc.with_ymd_and_hms(2023, 12, 20, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2023, 12, 25, 0, 0, 0).unwrap(),
        ));

        assert_eq!(rooms_events.len(), 4);

        let checkin_date = Utc.with_ymd_and_hms(2023, 12, 17, 0, 0, 0).unwrap();
        let checkout_date = Utc.with_ymd_and_hms(2023, 12, 20, 0, 0, 0).unwrap();

        // Expect available rooms are [2,3], since room 1 has event 
        let occupied_rooms: Vec<i32> = find_occupied_rooms(rooms_events, checkin_date, checkout_date);

        assert_eq!(occupied_rooms, [1, 2]);
    }
}

