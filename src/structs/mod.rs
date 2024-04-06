use mongodb::Client;
use serde::Deserialize;

pub struct AppData {
    pub mongodb_client: Client,
}

#[derive(Debug, Deserialize)]
pub struct Users {
    pub name: String,
    pub email: String,
    pub language: String,
    pub password: String,
}
