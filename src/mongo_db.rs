use dotenv::{dotenv, var};
use mongodb::options::ClientOptions;
use mongodb::Client;
use mongodb::Database;
use tokio::runtime::Runtime;

async fn client() -> Result<Database, mongodb::error::Error> {
    println!("client being loaded in");
    dotenv().ok();
    let key = var("KEY").unwrap();
    let mut client_options = ClientOptions::parse(key).await?;
    client_options.app_name = Some("My App".to_string());
    let client = Client::with_options(client_options);
    let db = client?.database("test");

    Ok(db)
}
pub fn mongo_time() {
    println!("initializing the mongo instance");
    let rt = Runtime::new().unwrap();
    let db = rt.block_on(client());

    match db {
        Ok(db) => println!("Connected to {}", db.name()),
        Err(e) => eprintln!("Failed to connect to the database: {}",e),
    }
}
