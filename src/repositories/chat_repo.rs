use mongodb::{bson::Array, Collection};



pub struct ChatRepo {
    pub chat_coll: Collection<Chat>,
}

pub struct Chat {
    participants: Array,
    messages: Array
}