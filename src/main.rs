use ::mongodb::Client;
use actix_web::{
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;
use serde_json::json;

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
            .route("/auth/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[derive(Debug, Deserialize)]
struct ExpectedData {
    password: String,
}

async fn login(body: web::Json<ExpectedData>, data: web::Data<AppData>) -> impl Responder {
    let password = &body.password;
    if password == "thisismypassword12" {
        //*return token
        HttpResponse::Ok().json(json!({"token":"this-is-your-token"}))
    } else {
        HttpResponse::BadRequest().body("this is not the password")
    }
}
