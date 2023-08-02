// use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoListData {
    // pub _id: Option<ObjectId>,
    pub title: String,
    pub job: String,
}

