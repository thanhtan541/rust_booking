use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Middleware",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self,
        _request: &'r Request<'_>,
        response: &mut Response<'r>) {
        response.set_header(Header::new(
            "access-control-allow-origin",
            "http://127.0.0.1:8081",
        ));
        response.set_header(Header::new(
            "access-control-allow-methods",
            "GET, PATCH, OPTIONS",
        ));
    }
}

