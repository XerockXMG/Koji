#[macro_use]
extern crate diesel;

use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

mod utils;
mod models;
mod queries;
mod routes;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let serve_from = if std::env::var("NODE_ENV") == Ok("development".to_string()) {
        "../dist"
    } else {
        "./dist"
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::new("%s | %r - %b bytes in %D ms (%a)"))
            .service(routes::config::config)
            .service(routes::gym::all)
            .service(routes::instance::all)
            .service(routes::pokestop::all)
            .service(routes::pokestop::area)
            .service(routes::pokestop::route)
            .service(routes::spawnpoint::all)
            .service(routes::spawnpoint::bound)
            .service(routes::spawnpoint::bootstrap)
            .service(
                Files::new("/", serve_from.to_string())
                    .index_file("index.html")
                    .prefer_utf8(true),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
