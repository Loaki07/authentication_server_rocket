use crate::models::user::*;
use color_eyre::Result as EyreResult;
use eyre::eyre;
use futures::stream::TryStreamExt;
use mongodb::{
    bson,
    bson::oid::ObjectId,
    bson::{doc, Document},
    error::Error,
    error::ErrorKind::OperationError,
    options::ClientOptions,
    results::DeleteResult,
    Client, Collection,
};
use serde_json::{json, Value};

pub const DATABASE_NAME: &str = "users";
pub const APP_NAME: &str = "authentication-server";

pub struct MongoUtil;

impl MongoUtil {
    pub async fn mongo_client() -> Result<Client, Error> {
        // Parse a connection string into an options struct.
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

        // Manually set an option
        client_options.app_name = Some("tasking-app".to_string());

        // Get a handle to the deployment.
        let client = Client::with_options(client_options)?;
        println!("Connected to mongodb");

        Ok(client)
    }

    pub async fn mongo_collection(name: &str) -> Result<Collection, Error> {
        let client = MongoUtil::mongo_client().await.unwrap();
        let db = client.database(APP_NAME);
        let collection = db.collection(name);
        Ok(collection)
    }

    pub async fn insert_one(data: RegisterUser) -> Result<Option<Value>, Error> {
        let db = MongoUtil::mongo_collection(DATABASE_NAME).await?;
        let insertable = bson::to_document(&data).unwrap();
        let bson_res = db.insert_one(insertable, None).await.unwrap();
        let res: ObjectId = bson::from_bson(bson_res.inserted_id).unwrap();

        let created_obj = Self::find_one(json!({ "_id": res }))
            .await
            .expect("Could not find document in db");
        Ok(created_obj)
    }

    pub async fn find_one(filter: Value) -> EyreResult<Option<Value>> {
        let db = MongoUtil::mongo_collection(DATABASE_NAME).await?;
        let insertable = bson::to_document(&filter).unwrap();
        let doc_res = db.find_one(insertable, None).await?;
        match doc_res {
            Some(document) => {
                let doc: Document =
                    bson::from_bson(bson::Bson::Document(document)).expect("could not decode");
                let res = json!(doc);
                println!("Find: {:#?}", res.clone());
                Ok(Some(res))
            }
            None => {
                let message = "Could not find document";
                return Err(eyre!(message));
            }
        }
    }

    pub async fn find_all() -> Result<Vec<Document>, Error> {
        let db = MongoUtil::mongo_collection(DATABASE_NAME).await?;
        let db_res = db.find(None, None).await.unwrap();
        let results: Vec<Document> = db_res.try_collect().await.unwrap();
        Ok(results)
    }

    pub async fn update_one(id: ObjectId, new_data: RegisterUser) -> Result<Option<Value>, Error> {
        println!("new_data: {:#?}", &new_data);
        let db = MongoUtil::mongo_collection(DATABASE_NAME).await?;
        let filter_json = json!({ "_id": id.clone() });
        let insertable_filter = bson::to_document(&filter_json).unwrap();
        println!("Updatable Task: :{:#?}", &new_data);
        let insertable_task = bson::to_document(&new_data).unwrap();
        println!("insertable_filter: {:#?}", insertable_filter.clone());

        let doc_res = db
            .update_one(insertable_filter, doc! { "$set": insertable_task }, None)
            .await?;

        println!("Updated {} document", doc_res.modified_count);

        let updated_obj = Self::find_one(json!({ "_id": id }))
            .await
            .expect("Could not find document in db");

        Ok(updated_obj)
    }

    pub async fn delete_one(id: ObjectId) -> Result<DeleteResult, Error> {
        let db = MongoUtil::mongo_collection(DATABASE_NAME).await?;
        let filter_json = json!({ "_id": id });
        let insertable_filter = bson::to_document(&filter_json).unwrap();
        println!("insertable_filter: {:#?}", insertable_filter.clone());
        db.delete_one(insertable_filter, None).await
    }
}
