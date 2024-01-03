use rocket::response::status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{delete, get, post, put, Responder};

#[derive(Responder)]
#[response(status = 418, content_type = "json")]
pub struct RawTeapotJson(&'static str);
#[derive(Serialize, Deserialize)]
pub struct NewBooking {
    start_date: String,
    end_date: String,
}
#[derive(Serialize, Deserialize)]
pub struct ExistedBooking {
    reference: u32,
    start_date: String,
    end_date: String,
}

#[get("/bookings/<ref_no>")]
pub fn show(ref_no: &str) -> String {
    format!("Showing booking info of {}!", ref_no)
}

#[post("/bookings", format = "json", data = "<booking>")]
pub fn create(booking: Json<NewBooking>) -> status::Accepted<String> {
    status::Accepted(Some(format!(
        "Created new booking from {} to {}",
        booking.start_date, booking.end_date
    )))
}

#[put("/bookings", format = "json", data = "<booking>")]
pub fn update(booking: Json<ExistedBooking>) -> status::Accepted<String> {
    status::Accepted(Some(format!("Updated booking of {}", booking.reference)))
}

#[delete("/bookings/<ref_no>")]
pub fn delete(ref_no: &str) -> String {
    format!("Deleted booking info of {}!", ref_no)
}

#[get("/bookings")]
pub fn fun() -> RawTeapotJson {
    RawTeapotJson("I'm am a TeaPot")
}

