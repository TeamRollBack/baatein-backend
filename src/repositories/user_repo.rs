use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

pub struct UserColl {
    pub user_coll: Collection<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub gender: Gender,
    pub dob: String,
}

impl UserColl {
    pub async fn init() -> Result<Self, ()> {
        let mut client_options = ClientOptions::parse("mongodb://172.26.192.1:27017/")
            .await
            .unwrap();
        client_options.app_name = Some("baatein".to_string());
        let client = Client::with_options(client_options).unwrap();

        Ok(Self {
            user_coll: client.database("baatein").collection("users"),
        })
    }

    pub async fn add_user(&self, user: User) -> bool {

        let uname = user.username.clone();

        if self.user_exists(uname).await {
            println!("username taken");
            false
        } else {
            self.user_coll.insert_one(user, None).await.expect("error adding user");
            println!("User added successfully");
            true
        }
    }

    pub async fn user_exists(&self, username: String) -> bool {
        let u = self.user_coll.find_one(doc! {"username": username}, None).await.expect("error");

        match u {
            Some(_u) => true,
            None => false
        }
    }
}
