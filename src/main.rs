extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
use controllers::booking_controller;
use rocket::{get, launch, routes};

mod controllers;

#[get("/hello")]
fn hello() -> String {
    "Hello".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello]).mount(
        "/",
        routes![
            booking_controller::show,
            booking_controller::create,
            booking_controller::update,
            booking_controller::delete,
            booking_controller::fun,
        ],
    )
}
