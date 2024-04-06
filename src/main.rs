use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mongodb::{options::ClientOptions, Client};
use std::error::Error;

async fn get_mongodb_client(connection_string: &str) -> Result<Client, Box<dyn Error>> {
    let client_options = ClientOptions::parse(connection_string).await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongodb_client = get_mongodb_client("mongodb://localhost:27017")
        .await
        .expect("good");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mongodb_client.clone()))
            .route("/list_databases", web::get().to(list_databases))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn list_databases(data: web::Data<Client>) -> impl Responder {
    match data.list_database_names(None, None).await {
        Ok(dbs) => HttpResponse::Ok().json(dbs),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
