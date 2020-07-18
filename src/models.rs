use serde::{Deserialize, Serialize};

use crate::errors::RestError;
use crate::errors::RestError::PokemonNotFound;
use crate::repository::pokemon::PokemonApiRepository;
use crate::repository::translator::TranslatorApiRepository;

#[derive(Serialize, Deserialize)]
pub struct Pokemon {
    pub name: String,
    pub description: String,
}

impl Pokemon {
    pub fn new(name: &str, description: &str) -> Pokemon {
        Pokemon { name: name.to_string(), description: description.to_string() }
    }
}

pub async fn get_shakespearean_description(name: &str, pokemon_repo: &PokemonApiRepository, translator_repo: &TranslatorApiRepository) -> Result<Pokemon, RestError> {
    let maybe_description = pokemon_repo.get_description(name).await?;

    match maybe_description {
        None => Err(PokemonNotFound(name.to_string())),
        Some(description) => translator_repo.get_shakespearean_translation(&description).await
    }.map(|d| Pokemon::new(name, &d))
}