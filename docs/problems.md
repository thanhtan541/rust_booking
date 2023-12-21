Problems: How does customer find available room from date x to date y?

Solutions:
1 - Load all room's events that have start date or end date within date x and date y
2 - If a room event overlaps with search date's range, mark its room as occupied
3 - Load all rooms and exclude all occupied

What do we need in this solution?
- Room
    - Id
- Room Event: Cleaning, Reparing, Reserved
    - Id
    - room_id (fk)
    - start_date
    - end_date

Example:
- Searching for date 10 to 15
- There are two rooms
- Room 1st booked from 8 to 11
- Room 2nd booked from 11 to 14
- Result: no available rooms

Problems: How deos customer make a booking?
Solutions:
1 - Pick an availability room
2 - Show detail booking: price, duration ...
3 - Confirm

What do we need in this solution?
- Room
    - Id
- Booking Snapshot: Avoid general names like order due to confuse with shopping site
    - Id
    - item_id (fk) -> room
    - start_date
    - end_date
- Booking Event:
    - Id
    - booking_number (fk) -> bookingsnapshot
    - action


Booking Even type: Created, Updated, Done 
