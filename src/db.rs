use mongodb::{options::ClientOptions, Client, Database};

#[derive(Clone)]
pub struct DB {
    pub db: Database,
}

impl DB {
    pub async fn init() -> Result<Self, ()> {
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017/")
            .await
            .unwrap();
        client_options.app_name = Some("baatein".to_string());
        let client = Client::with_options(client_options).unwrap();

        Ok(Self {
            db: client.database("baatein")
        })
    }

    pub async fn get_db(&self) -> Database {
        self.db.clone()
    }
}