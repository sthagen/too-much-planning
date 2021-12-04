#[path = "../v1/mod.rs"]
mod v1;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use v1::{dbaccess, errors, handler, model, routes, state};
use routes::{app_config, build_config};
use sqlx::postgres::PgPool;
use std::env;

use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host_port = env::var("HOST_PORT").expect("HOST:PORT address not set in .env file");
    println!("Manager listening on: {}", &host_port);
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");
    let db_pool = PgPool::new(&database_url).await.unwrap();
    let shared_data = web::Data::new(state::AppState { db: db_pool });

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();

        App::new()
            .data(tera)
            .app_data(shared_data.clone())
            .configure(build_config)
            .configure(app_config)
    })
    .bind(&host_port)?
    .run()
    .await
}
