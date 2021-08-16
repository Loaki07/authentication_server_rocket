use crate::config::crypto::CryptoService;
use crate::models::user::*;
use rocket::{
    form::{Form, Strict},
    http::Status,
    response::status,
};
use serde_json::{json, Value};

#[post("/auth/sign-in", data = "<user>")]
pub async fn sign_in(
    user: Form<Strict<LoginUser>>,
) -> Result<status::Custom<Value>, status::Custom<Value>> {
    println!("Sign-in: {:#?}", user);
    let message = json!({"success": true, "data": "Login Successful"});
    Ok(status::Custom(Status::Ok, message))
}

#[post("/auth/sign-up", data = "<user>")]
pub async fn sign_up(
    user: Form<Strict<RegisterUser>>,
) -> Result<status::Custom<Value>, status::Custom<Value>> {
    let hasher = CryptoService::new();
    let password_hash = hasher.hash_password(user.password.clone()).await.unwrap();
    println!("password_hash : {:#?}", password_hash);
    let message = json!({"success": true, "data": "User Registration Successful"});
    Ok(status::Custom(Status::Ok, message))
}
