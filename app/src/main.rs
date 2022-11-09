use actix_web::web;
use actix_web::{App, HttpServer};
use app::db;
use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env var not specified!");
    let listen = env::var("LISTEN").unwrap_or("127.0.0.1".into());
    let port = env::var("PORT").unwrap_or("5000".into());
    let port = port
        .parse()
        .expect(format!("Error parsing port number PORT={}", port).as_str());

    println!("DATABASE_URL: {}", database_url);

    let db = db::init_database(&database_url)
        .await
        .expect("failed connecting database!");

    println!("listening on http://{}:{}", listen, port);
    HttpServer::new(move || {
        App::new()
            .configure(app::route::route_configure)
            .app_data(web::Data::new(db.clone()))
    })
    .bind((listen, port))?
    .run()
    .await
}
