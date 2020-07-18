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
