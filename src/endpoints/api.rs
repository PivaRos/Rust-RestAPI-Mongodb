use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Temp {
    pub id: u32,
    pub name: String,
}

pub async fn index() -> impl Responder {
    web::Json(Temp {
        id: 1,
        name: "this is response from api route ".to_string(),
        // populate other fields...
    })
}
