use mongodb::{
    bson::{oid::ObjectId, Document},
    Collection,
};
use serde::{Deserialize, Serialize};

use crate::db::DB;

pub struct MessageRepo {
    pub message_coll: Collection<Message>,
    pub gen_coll: Collection<Document>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    sender: ObjectId,
    message: String,
}

impl MessageRepo {
    pub async fn init(db: DB) -> Result<Self, ()> {
        Ok(Self {
            message_coll: db.get_db().await.collection("messages"),
            gen_coll: db.get_db().await.collection("messages"),
        })
    }

    pub async fn create_message(&self, message: Message) {
        self.message_coll
            .insert_one(message, None)
            .await
            .expect("error creating message");
    }
}
