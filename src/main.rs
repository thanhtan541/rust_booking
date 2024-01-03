extern crate rocket;
extern crate rocket_contrib;

use rocket::{get, launch, routes};

mod middleware;
mod controllers;

use controllers::*;
use middleware::cors::Cors;

#[get("/hello")]
fn hello() -> String {
    "Hello".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount(
            "/api",
            routes![
                booking_controller::show,
                booking_controller::create,
                booking_controller::update,
                booking_controller::delete,
                booking_controller::fun,
                room_controller::list,
            ],
        )
        .attach(Cors)
}

