#[macro_use] extern crate rocket;

#[get("/hello")]
fn hello() -> String {
    format!("Hello")
}

#[launch]
fn rocket() -> _ {
    use rocket::fairing::AdHoc;
    rocket::build()
        .mount("/", routes![hello])
        .attach(AdHoc::on_request("Compatibility Normalizer", |req, _| Box::pin(async move {
            if !req.uri().is_normalized() {
                let normal = req.uri().clone().into_normalized();
                warn!("Incoming request URI was normalized for compatibility.");
                info_!("{} -> {}", req.uri(), normal);
                req.set_uri(normal);
            }
        })))
}

