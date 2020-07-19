use actix_web::{App, HttpServer, web};

use handlers::get_pokemon;

use crate::repository::Repositories;

mod errors;

mod handlers;
mod models;
mod repository;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(Repositories::default())
            .route("/pokemon/{pokemon_name}", web::get().to(get_pokemon))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}