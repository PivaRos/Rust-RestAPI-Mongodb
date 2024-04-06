use std::result;

use actix_web::{
    web::{self, Json},
    HttpResponse, Responder,
};
use mongodb::{bson::doc, Collection};
use serde::{de::value, Deserialize};
use serde_json::json;

use crate::structs::{self, AppData, Users};

#[derive(Debug, Deserialize)]
pub struct ExpectedData {
    password: String,
    email: String,
}

pub async fn login(body: web::Json<ExpectedData>, data: web::Data<AppData>) -> impl Responder {
    let users: Collection<structs::Users> =
        data.mongodb_client.database("test1").collection("users");

    let password = &body.password;
    let email = &body.email;

    let filter = doc! {
        "password": password,
        "email": email
    };

    // Query the database to find the user matching the provided password
    match users.find_one(filter, None).await {
        Ok(Some(user)) => {
            let name = &user.name;
            HttpResponse::Ok().json(json!({"name":name}))
        } // User found, return it
        Ok(None) => HttpResponse::NotFound().body("User not found"), // User not found
        Err(_) => HttpResponse::InternalServerError().finish(),      // Error occurred
    }
}
