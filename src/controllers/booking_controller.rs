use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post};

#[get("/api/bookings/<ref_no>")]
pub fn show(ref_no: &str) -> String {
    format!("Showing booking info of {}!", ref_no)
}

#[derive(Serialize, Deserialize)]
pub struct NewBooking {
    start_date: String,
    end_date: String,
}

#[post("/api/bookings", format = "json", data = "<booking>")]
pub fn create(booking: Json<NewBooking>) -> String {
    format!(
        "Created new booking from {} to {}",
        booking.start_date, booking.end_date
    )
}
