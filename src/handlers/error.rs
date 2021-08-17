use serde_json::Value;

#[derive(Debug)]
pub enum AuthenticationError {
    MongoError(mongodb::error::Error),
    UserAlreadyExists(Value),
    DbError(String)
}

impl From<mongodb::error::Error> for AuthenticationError {
    fn from(err: mongodb::error::Error) -> Self {
        AuthenticationError::MongoError(err)
    }
}
