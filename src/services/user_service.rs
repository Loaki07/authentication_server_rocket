use crate::handlers::error::AuthenticationError;
use crate::models::user::{LoginUser, RegisterUser};
use crate::utils::mongo_util::MongoUtil;
use serde_json::{json, Value};

pub struct UserService;

impl UserService {
    pub async fn register(user: RegisterUser) -> Result<Value, AuthenticationError> {
        // Check if the user is already present in db
        match MongoUtil::find_one(json!({"email_id": user.email_id})).await {
            Ok(data) => Err(AuthenticationError::UserAlreadyExists(data.unwrap())),
            Err(_) => {
                // If user does not exist, create new user
                let new_user = MongoUtil::insert_one(user.clone()).await?.unwrap();
                Ok(new_user)
            }
        }
    }

    pub async fn login(user: LoginUser) -> Result<Value, AuthenticationError> {
        // Check if the user is already present in db
        let found_user = MongoUtil::find_one(json!({"email_id": user.username}))
            .await
            .map_err(|err| AuthenticationError::DbError(err.to_string()));

        let message =
            json!({"success": true, "message": "User Registration Successful", "data": "res"});
        Ok(message)
    }
}
