use ::mongodb::Client;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod mongodb;

struct AppData {
    mongodb_client: Client,
}

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
            .route("/list_databases", web::get().to(list_databases))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn list_databases(data: web::Data<AppData>) -> impl Responder {
    match data.mongodb_client.list_database_names(None, None).await {
        Ok(dbs) => HttpResponse::Ok().json(dbs),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
