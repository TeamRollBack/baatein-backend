use mongodb::{
    bson::{doc, oid::ObjectId, Bson, Document},
    Collection,
};
use serde::{Deserialize, Serialize};

use crate::db::DB;

pub struct ChatRepo {
    pub chat_coll: Collection<Chat>,
    pub gen_coll: Collection<Document>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participants {
    pub p1: ObjectId,
    pub p2: ObjectId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    participants: Participants,
    messages: Vec<Bson>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub u1: String,
    pub u2: String,
}

impl ChatRepo {
    pub async fn init(db: DB) -> Result<Self, ()> {
        Ok(Self {
            chat_coll: db.get_db().await.collection("chats"),
            gen_coll: db.get_db().await.collection("chats"),
        })
    }

    pub async fn create_chat(&self, participants: Participants) {
        let temp_chat = Chat {
            participants: participants,
            messages: Vec::new(),
        };
        self.chat_coll
            .insert_one(temp_chat, None)
            .await
            .expect("error creating chat");
    }

    pub async fn add_msg(&self, participants: Participants, msg_id: Bson) {
        let filter = doc! {
            "$or": [
                    {"participants": {"p1": participants.p1, "p2": participants.p2}},
                    {"participants": {"p1": participants.p2, "p2": participants.p1}}
                ]
        };

        self.chat_coll.find_one_and_update(filter, doc!{"$push": {"messages": msg_id }}, None).await.unwrap();
    }

    pub async fn get_chats(&self, participants: Participants) -> Vec<Bson> {
        let filter = doc! {
            "$or": [
                    {"participants": {"p1": participants.p1, "p2": participants.p2}},
                    {"participants": {"p1": participants.p2, "p2": participants.p1}}
                ]
        };

        match self.chat_coll.find_one(filter, None).await.unwrap() {
            Some(chat) => {
                chat.messages
            },
            None => {
                Vec::new()
            }
        }
    }
}
