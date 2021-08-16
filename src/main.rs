#[macro_use]
extern crate rocket;

mod controller;
mod models;
mod services;

use rocket::{http::Status, response::status};
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

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![api_home])
        .register("/", catchers![not_found])
}
