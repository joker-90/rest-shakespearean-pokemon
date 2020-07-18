use actix_web::{HttpResponse, Responder, web};
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

    use super::*;

    #[actix_rt::test]
    async fn test_pokemon_get() {
        let mut app = test::init_service(App::new()
            .data(Repositories::default())
            .route("/pokemon/{pokemon_name}", web::get().to(get_pokemon))).await;
        let req = test::TestRequest::with_uri("/pokemon/bulbasaur").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}