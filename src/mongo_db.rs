use dotenv::{dotenv, var};
use mongodb::bson::{doc, Document};
use mongodb::options::ClientOptions;
use mongodb::Client;
use mongodb::Database;
use tokio::runtime::Runtime;

async fn export(db: Database) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<Document>("books");

    let doc = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];

    collection.insert_many(doc, None).await?;

    Ok(())
}

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
    let db_result = rt.block_on(client());

    match db_result {
        Ok(db) => {
            println!("Connected to {}", db.name());

            println!("testing the export function");

            let results = rt.block_on(export(db));

            match results {
                Ok(_) => println!("export has occured, very nice"),
                Err(e) => eprintln!("failed to export the data: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to connect to the database: {}", e),
    }
}
