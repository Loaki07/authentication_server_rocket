#[macro_use]
extern crate rocket;
#[macro_use]
extern crate validator_derive;

mod config;
mod controller;
mod models;
mod services;
mod utils;

use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{Header, Status},
    response::status,
    Request, Response,
};
use serde_json::{json, Value};

#[get("/")]
fn api_home() -> status::Custom<Value> {
    let message = json!({"success": true, "data": "Authentication Server"});
    status::Custom(Status::Ok, message)
}

#[catch(404)]
fn not_found() -> status::Custom<Value> {
    let message = json!({ "success": false, "data": "Not found!" });
    status::Custom(Status::NotFound, message)
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![api_home, controller::sign_in, controller::sign_up],
        )
        .attach(CORS)
        .register("/", catchers![not_found])
}
