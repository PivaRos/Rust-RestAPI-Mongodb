use ::mongodb::Client;
use actix_web::{
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;
use serde_json::json;

mod auth;
mod mongodb;
mod structs;

use crate::structs::AppData;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongodb_client = mongodb::get_mongodb_client("mongodb://localhost:27017")
        .await
        .expect("good");

    let app_data = web::Data::new(AppData {
        mongodb_client: mongodb_client.clone(),
        // Add more data as needed
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/auth/login", web::post().to(auth::login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
