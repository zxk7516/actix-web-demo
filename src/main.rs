#[macro_use]
extern crate diesel;
use actix_web::{web, App, HttpServer};

mod models;
mod routes;
mod schema;

use diesel::r2d2::{self, ConnectionManager};
//use diesel::SqliteConnection as DbConnection;
//use diesel::MysqlConnection as DbConnection;
use diesel::PgConnection as DbConnection;

pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let database_pool = Pool::builder()
        .build(ConnectionManager::<DbConnection>::new(database_url))
        .expect("database pool failed");

    HttpServer::new(move || {
        App::new()
            .data(database_pool.clone())
            .route("/", web::get().to(routes::home))
            .route("/addLink", web::post().to(routes::add_link))
            .route("/getLinks", web::get().to(routes::get_links))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
