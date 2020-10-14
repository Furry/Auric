use mongodb::{Client, options::ClientOptions, Database};
use mongodb::bson::Document;
use futures::stream::StreamExt;

pub async fn client(uri: &'static str) -> Result<Client, anyhow::Error> {
    let mut client_options = ClientOptions::parse(uri).await?;
    client_options.app_name = Some("ShareX".to_string());
    let client = Client::with_options(client_options)?;
    println!("Initing DBs...");
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name)
    }
    Ok(client)
}

//#[derive(Clone)]
pub struct MongoEngine {
    pub client: Client,
    pub db: Database
}

/* Placeholder for MongoDB Result
pub struct User {
    pub token: String,
    pub urls: Vec<String>
}
*/

impl MongoEngine {
    //pub fn move_db(database: String) {
    //    client.database()
    //}
    #[allow(dead_code)]
    pub fn debug(&self) {
        println!("Debugged!");
    }

    #[allow(dead_code)]
    pub async fn insert(&self, collection: &str, input: Document) -> Result<(), anyhow::Error> {
        let collection = self.db.collection(collection);
        collection.insert_one(input, None).await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn query(&self, collection: &str, key: &str, val: &str) -> Result<Vec<Document>, anyhow::Error> {
        let mut cursor = self.db
            .collection(collection)
            .find(None, None)
            .await?;
        let mut result: Vec<Document> = Vec::new();
        while let Some(doc_) = cursor.next().await {
            let doc = doc_?;
            let gotkey = *&doc.get(key).unwrap().as_str().unwrap();
            if gotkey == val {
                result.push(doc);
            }
        }
        Ok(result)
    }
}