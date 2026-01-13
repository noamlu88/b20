mod db;
mod models;
mod controllers;
mod routes;

use actix_web::{App, HttpServer, web};
use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::connect_db().await.expect("DB failed");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(customer::customer_routes)
            .configure(account::account_routes)
            .configure(transaction::transaction_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
