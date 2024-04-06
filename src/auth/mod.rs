use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

use crate::structs::AppData;

#[derive(Debug, Deserialize)]
pub struct ExpectedData {
    password: String,
}

pub async fn login(body: web::Json<ExpectedData>, data: web::Data<AppData>) -> impl Responder {
    let password = &body.password;
    if password == "thisismypassword12" {
        //*return token
        HttpResponse::Ok().json(json!({"token":"this-is-your-token"}))
    } else {
        HttpResponse::BadRequest().body("this is not the password")
    }
}
