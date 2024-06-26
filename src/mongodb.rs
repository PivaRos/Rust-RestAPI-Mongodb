use mongodb::{options::ClientOptions, Client};
use std::error::Error;

pub async fn get_mongodb_client(connection_string: &str) -> Result<Client, Box<dyn Error>> {
    let client_options = ClientOptions::parse(connection_string).await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}
