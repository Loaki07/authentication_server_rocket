use crate::config::crypto::CryptoService;
use crate::handlers::error::AuthenticationError;
use crate::models::user::{LoginUser, RegisterUser, User};
use crate::utils::mongo_util::MongoUtil;
use serde_json::json;

pub struct UserService;

impl UserService {
    pub async fn register(user: RegisterUser) -> Result<User, AuthenticationError> {
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

    pub async fn login(user: LoginUser) -> Result<bool, AuthenticationError> {
        // Check if the user is already present in db
        let found_user = MongoUtil::find_one(json!({"email_id": user.username}))
            .await
            .map_err(|err| AuthenticationError::DbError(err.to_string()))
            .and_then(|data| Ok(data.unwrap()));

        // Verify passwords
        let verifier = CryptoService::new();
        match verifier
            .verify_password(user.password, found_user.unwrap().password.unwrap())
            .await
        {
            Ok(is_verified) => {
                return if is_verified {
                    Ok(is_verified)
                } else {
                    Err(AuthenticationError::PasswordMismatch(
                        "Password Does Not Match".to_owned(),
                    ))
                }
            }
            Err(e) => Err(AuthenticationError::LoginError(e.to_string())),
        }
    }
}
