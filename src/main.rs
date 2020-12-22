#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::MysqlConnection;
use dotenv::dotenv;
use std::env;

use actix_web::{get, web, App, HttpServer, Responder};
use self::models::Wallet;
pub mod schema;
pub mod models;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[get("/wallet/{address}")]
async fn wallet(web::Path(add): web::Path<String>) -> impl Responder {
    use schema::wallets::dsl::*;

    let connection = establish_connection();
    let result = wallets.filter(address.eq(add))
        .first::<Wallet>(&connection)
        .expect("Error loading wallets");

    web::Json(Wallet {
        id: result.id,
        name: result.name,
        address: result.address,
        parties: result.parties,
        threshold: result.threshold,
        balance: result.balance
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(wallet))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
