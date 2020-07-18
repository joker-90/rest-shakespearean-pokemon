use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

use crate::models::Pokemon;

#[derive(Clone, Deserialize)]
pub struct GetPokemonInfo {
    pokemon_name: String
}

pub async fn get_pokemon(info: web::Path<GetPokemonInfo>) -> impl Responder {
    HttpResponse::Ok().json(Pokemon::new(&info.pokemon_name, "sss"))
}

#[cfg(test)]
mod tests {
    use actix_web::{App, test, web};

    use super::*;

    #[actix_rt::test]
    async fn test_pokemon_get() {
        let mut app = test::init_service(App::new()
            .route("/pokemon/{pokemon_name}", web::get().to(get_pokemon))).await;
        let req = test::TestRequest::with_uri("/pokemon/bulbasaur").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}