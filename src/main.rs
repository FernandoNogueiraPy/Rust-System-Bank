mod database {pub mod connect_database;}

use std::collections::HashMap;
use crate::database::connect_database::ConfigDatabase;
use mongodb::{error::Error, Client};


#[tokio::main]
async fn main() -> Result<(), Error> {
    // Crie uma instância de `Config_Database` com os detalhes de conexão
    let connect_db = ConfigDatabase::new(
        "mongodb://localhost:27017",
        "test",
        "test_collection",
    );

    // Conecte-se ao banco de dados
    let collection = connect_db.connect_to_mongodb().await?;

    let mut document = HashMap::new();
    document.insert("name".to_string(), "John".to_string());
    document.insert("age".to_string(), "30".to_string());

    // Inserindo documento
    match Config_Database::insert_document(&collection, document.clone()).await {
        Ok(()) => println!("Documento inserido com sucesso."),
        Err(e) => eprintln!("Erro ao inserir documento: {}", e),
    }

    // Pesquisando documentos
    let mut filter = HashMap::new();
    filter.insert("name".to_string(), "John".to_string());

    match Config_Database::search_documents(&collection, filter.clone()).await {
        Ok(()) => println!("Pesquisa concluída com sucesso."),
        Err(e) => eprintln!("Erro ao pesquisar documentos: {}", e),
    }

    Ok(())
}
