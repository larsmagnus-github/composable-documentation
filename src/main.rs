mod routes;

use actix_files::Files;
use actix_web::{App, HttpServer};
use actix_cors::Cors;
use routes::convert_url;
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at: http://127.0.0.1:8080");

    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(convert_url)
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}