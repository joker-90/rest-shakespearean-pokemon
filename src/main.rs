use actix_web::{App, HttpServer, web};

use handlers::get_pokemon;

mod errors;

mod handlers;
mod models;
mod repository;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/pokemon/{pokemon_name}", web::get().to(get_pokemon))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}