use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};
use std::env;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let env_path = env::current_dir()?.join(".env");
    dotenv::from_path(&env_path).ok();
    let user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let password = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");

    let uri = format!("mongodb://{}:{}@localhost:27017", user, password);
    let client = Client::with_uri_str(uri).await?;

    let database = client.database("shop");
    let my_coll: Collection<Document> = database.collection("products");
    let my_product = my_coll.find_one(doc! { "name": "Nhat Vu" }, None).await?;

    println!("Found a product:\n{:#?}", my_product);

    let docs = vec![
        doc! { "title": "Friends With Money", "runtime": 88 },
        doc! { "title": "Please Give", "runtime": 90 },
        doc! { "title": "You Hurt My Feelings", "runtime": 93 },
    ];
    my_coll.insert_many(docs, None).await?;
    let mut cursor = my_coll.find(doc! {}, None).await?;

    while cursor.advance().await? {
        println!("{:?}", cursor.deserialize_current()?);
    }

    Ok(())
}
