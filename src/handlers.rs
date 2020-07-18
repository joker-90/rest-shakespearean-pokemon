use actix_web::web;
use actix_web::web::{Data, Json};
use serde::Deserialize;

use crate::errors::RestError;
use crate::models::{get_shakespearean_description, Pokemon};
use crate::repository::Repositories;

#[derive(Clone, Deserialize)]
pub struct GetPokemonInfo {
    pokemon_name: String
}

pub async fn get_pokemon(info: web::Path<GetPokemonInfo>, repositories: Data<Repositories>) -> Result<Json<Pokemon>, RestError> {
    let pokemon = get_shakespearean_description(&info.pokemon_name, &repositories.pokemon, &repositories.translator).await?;

    Ok(Json(pokemon))
}


#[cfg(test)]
mod tests {
    use actix_web::{App, test, web};
    use actix_web::http::StatusCode;
    use actix_web::test::read_body;
    use serde_json::Value;

    use super::*;

    #[actix_rt::test]
    async fn test_pokemon_get() {
        let mut app = test::init_service(App::new()
            .data(Repositories::default())
            .route("/pokemon/{pokemon_name}", web::get().to(get_pokemon))).await;
        let req = test::TestRequest::with_uri("/pokemon/bulbasaur").to_request();

        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());

        let body = read_body(resp).await;

        let result_pokemon = serde_json::from_slice::<Pokemon>(&body).unwrap();

        assert_eq!(result_pokemon.name, "bulbasaur");
        assert!(!result_pokemon.description.is_empty())
    }

    #[actix_rt::test]
    async fn test_unknown_pokemon_get() {
        let mut app = test::init_service(App::new()
            .data(Repositories::default())
            .route("/pokemon/{pokemon_name}", web::get().to(get_pokemon))).await;
        let req = test::TestRequest::with_uri("/pokemon/unknown_pokemon").to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        let body = read_body(resp).await;

        let result_error = serde_json::from_slice::<Value>(&body).unwrap();

        assert_eq!(result_error["message"].as_str(), Some("Pokemon with name unknown_pokemon not found!"));
    }
}