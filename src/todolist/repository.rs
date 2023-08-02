use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{InsertOneResult, UpdateResult, DeleteResult},
    Client, Collection
};
use super::models::TodoListData;

pub struct MongoRepo {
    col: Collection<TodoListData>
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGODB_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Gagal loading env variable")
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("todolist-app");
        let col = db.collection("todolist_data");
        MongoRepo { col }
    }

    pub async fn create_data(&self, new_todolist: TodoListData) -> Result<InsertOneResult, Error> {
        let new_doc = TodoListData {
            // id: None,
            title: new_todolist.title,
            job: new_todolist.job,
        };
        let data_create = self
            .col.insert_one(new_doc, None).await.ok().expect("Gagal membuat data!");
        Ok(data_create)
    }

    pub async fn get_detail_data(&self, id: &String) -> Result<TodoListData, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let data_detail = self
            .col.find_one(filter, None).await.ok().expect("Gagal mengambil detail data!");
        Ok(data_detail.unwrap())
    }

    pub async fn update_data(&self, id: &String, data_update: TodoListData) -> Result<UpdateResult, Error> {
        let obj_id  = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set": {
                // "id": data_update.id,
                "title": data_update.title,
                "job": data_update.job
            }
        };
        let data_update = self
            .col.update_one(filter, new_doc, None).await.ok().expect("Gagal memperbarui data!");
        Ok(data_update)
    }

    pub async fn delete_data(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let data_detail = self
            .col.delete_one(filter, None).await.ok().expect("Gagal menghapus data!");
        Ok(data_detail)
    }
    
}




