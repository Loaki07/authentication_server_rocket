use crate::config::crypto::CryptoService;
use crate::models::user::*;
use crate::services::user_service::UserService;
use crate::utils::mongo_util::MongoUtil;
use rocket::serde::json::Json;
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
    let result = UserService::login(user.into_inner().into_inner()).await.map_err(|e| {
        let message = json!({"success": false, "message": format!("Login Failed with error: {:#?}", e)});
        return status::Custom(Status::NotImplemented, message);
    }).and_then(|res| {
        let message = json!({"success": true, "message": "Login Successful", "data": res});
        return Ok(status::Custom(Status::Ok, message));
    });
    result
}

#[post("/auth/sign-up", data = "<user>")]
pub async fn sign_up(
    mut user: Form<Strict<RegisterUser>>,
) -> Result<status::Custom<Value>, status::Custom<Value>> {
    let hasher = CryptoService::new();
    let password_hash = hasher.hash_password(user.password.clone()).await.unwrap();
    user.password = password_hash;
    println!("password_hash : {:#?}", user.clone());

    let result = UserService::register(user.into_inner().into_inner()).await.map_err(|e| {
        let message = json!({"success": false, "message": format!("User Registration Failed with error: {:#?}", e)});
        return status::Custom(Status::NotImplemented, message);

    }).and_then(|res| {
        let message = json!({"success": true, "message": "User Registration Successful", "data": res});
        return Ok(status::Custom(Status::Ok, message));
    });
    result
}

#[post("/auth/find-user", data = "<user>")]
pub async fn find_user(user: Json<Value>) -> Result<status::Custom<Value>, status::Custom<Value>> {
    MongoUtil::find_one(json!(user.into_inner())).await.map_err(|err| {
        let message = json!({"success": false, "message": format!("Find User Failed with error: {:#?}", err)});
        return status::Custom(Status::InternalServerError, message);

    }).and_then(|data| {
        let message = json!({"success": true, "message": "Found User", "data": data});
        return Ok(status::Custom(Status::Ok, message))
    })
}
