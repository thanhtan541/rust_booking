extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, launch, post, routes};

mod models;
pub mod schema;

use rocket_dyn_templates::{context, Template};
use std::env;
pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Serialize, Deserialize)]
struct NewBooking {
    start_date: String,
    end_date: String,
}

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/api/booking", format = "json", data = "<booking>")]
fn create_booking(booking: Json<NewBooking>) -> Result<Created<Json<NewBooking>>> {
    use self::schema::bookings::dsl::*;
    use models::Booking;
    let mut connection = establish_connection_pg();

    let new_booking = Booking {
        id: 2,
        start_date: booking.start_date.to_string(),
        end_date: booking.end_date.to_string(),
    };

    diesel::insert_into(bookings)
        .values(&new_booking)
        .execute(&mut connection)
        .expect("Error saving new booking");

    Ok(Created::new("/").body(booking))
}

#[get("/api/bookings")]
fn index() -> Template {
    use self::models::Booking;

    let connection = &mut establish_connection_pg();
    let results = self::schema::bookings::dsl::bookings
        .load::<Booking>(connection)
        .expect("Error loading bookings");
    Template::render(
        "bookings",
        context! {bookings: &results, count: results.len()},
    )
}

#[get("/hello")]
fn hello() -> String {
    "Hello".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![create_booking])
        .mount("/", routes![index])
        .mount("/", routes![hello])
        .attach(Template::fairing())
}
