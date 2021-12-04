use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../v1/dbaccess/mod.rs"]
mod dbaccess;
#[path = "../v1/errors.rs"]
mod errors;
#[path = "../v1/handlers/mod.rs"]
mod handlers;
#[path = "../v1/models/mod.rs"]
mod models;
#[path = "../v1/routes.rs"]
mod routes;
#[path = "../v1/state.rs"]
mod state;

use errors::ManageBuildsError;
use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");
    let db_pool = PgPool::new(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "OK::Request".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                ManageBuildsError::InvalidInput("Invalid Json input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(build_routes)
            .configure(tutor_routes)
    };

    let host_port = env::var("HOST_PORT").expect("HOST:PORT address not set in .env file");
    HttpServer::new(app).bind(&host_port)?.run().await
}
