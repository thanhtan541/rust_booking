use rocket::{catch, catchers, get, launch, routes, Request};

mod controllers;
mod middleware;

use controllers::*;
use middleware::cors::Cors;

#[get("/hello")]
fn hello() -> String {
    "Hello".to_string()
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
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
        .register("/", catchers![not_found])
}
