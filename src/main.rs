use std::env;
use futures::TryStreamExt;
use mongodb::bson::Document;
use mongodb::Client;
use mongodb::options::{ClientOptions};
use mongodb::bson::doc;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<(), mongodb::error::Error> {
    dotenv().ok();
    let database_url = env::var("MONGODB_URL").expect("需要配置环境变量MONGODB_URL指明mongodb连接url");
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    println!("Hello, world!");
}
