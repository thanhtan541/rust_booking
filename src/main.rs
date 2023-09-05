extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
use controllers::booking_controller;
// use diesel::pg::PgConnection;
// use diesel::prelude::*;
// use dotenvy::dotenv;
use rocket::{get, launch, routes};

mod controllers;
mod models;
pub mod schema;

use rocket_dyn_templates::{context, Template};
use std::env;
// pub fn establish_connection_pg() -> PgConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }

// type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

// #[post("/api/booking", format = "json", data = "<booking>")]
// fn create_booking(booking: Json<NewBooking>) -> Result<Created<Json<NewBooking>>> {
//     use self::schema::bookings::dsl::*;
//     use models::Booking;
//     let mut connection = establish_connection_pg();

//     let new_booking = Booking {
//         id: 2,
//         start_date: booking.start_date.to_string(),
//         end_date: booking.end_date.to_string(),
//     };

//     diesel::insert_into(bookings)
//         .values(&new_booking)
//         .execute(&mut connection)
//         .expect("Error saving new booking");

//     Ok(Created::new("/").body(booking))
// }

#[get("/hello")]
fn hello() -> String {
    "Hello".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount(
            "/",
            routes![booking_controller::show, booking_controller::create,],
        )
        .attach(Template::fairing())
}
