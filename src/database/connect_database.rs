use mongodb::{bson::doc, error::Error, Client, Collection};
use std::collections::HashMap;






impl ConfigDatabase{

    pub fn new(connection_database: String, name_database: String, collection_name: String) -> Self {
        ConfigDatabase {
            connection_database,
            name_database,
            collection_name,
        }
    }

    // Insert documents in collections.
    async fn insert_document(&self,document: HashMap<String, String>) -> Result<(), Error> {

        collection.insert_one(document, None).await?;
        Ok(())
    }

    // Search documents in collections.
    async fn search_documents(&self,filter: HashMap<String, String>){

        let cursor = collection.find(filter, None).await?;
        for result in cursor { if let Ok(document) = result { println!("{:?}", document);}}

    }

    // Create connection with mongodb
    pub async fn connect_to_mongodb(&self) -> Result<Collection, Error> {

        let client = Client::with_uri_str(&self.connection_database).await?;
        let db = client.database(&self.name_database);
        let collection= db.collection(&self.collection_name);
    
        Ok(collection)
    }
}



pub struct ConfigDatabase {
    pub connection_database: String,
    pub name_database: String,
    pub collection_name: String,
}


