use crate::models::user::User;

#[derive(Debug)]
pub enum AuthenticationError {
    MongoError(mongodb::error::Error),
    UserAlreadyExists(User),
    DbError(String),
    PasswordMismatch(String),
    LoginError(String)
}

impl From<mongodb::error::Error> for AuthenticationError {
    fn from(err: mongodb::error::Error) -> Self {
        AuthenticationError::MongoError(err)
    }
}
