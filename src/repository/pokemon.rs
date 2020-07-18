use std::borrow::Borrow;

use actix_web::client::{Client, Connector};
use actix_web::error::Error;
use actix_web::http::StatusCode;
use openssl::ssl::{SslConnector, SslMethod};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::RestError;

pub struct PokemonApiRepository {
    client: Client
}

impl PokemonApiRepository {
    pub fn default() -> PokemonApiRepository {
        let builder = SslConnector::builder(SslMethod::tls())
            .unwrap();

        let client = Client::build()
            .connector(Connector::new().ssl(builder.build()).finish())
            .finish();

        PokemonApiRepository { client }
    }

    pub async fn get_description(&self, name: &str) -> Result<Option<String>, RestError> {
        let maybe_species_url = self.get_species_url(name)
            .await
            .map_err(|e| RestError::UpstreamError(e.to_string()))?;

        match maybe_species_url {
            None => Ok(None),
            Some(species_url) => self.get_species_en_flavor_text(&species_url).await
                .map_err(|e| RestError::UpstreamError(e.to_string())),
        }
    }

    async fn get_species_url(&self, name: &str) -> Result<Option<String>, Error> {
        let url = format!("https://pokeapi.co/api/v2/pokemon/{}", name);

        let mut response = self.client.get(url)
            .send()
            .await
            .map_err(Error::from)?;

        if let StatusCode::NOT_FOUND = response.status() {
            return Ok(None);
        }

        let body = response.body().await?;

        let json = serde_json::from_slice::<Value>(&body)
            .map_err(|e| Error::from(e))?;

        Ok(PokemonApiRepository::extract_species(&json))
    }

    async fn get_species_en_flavor_text(&self, url: &str) -> Result<Option<String>, Error> {
        let mut response = self.client.get(url)
            .send()
            .await
            .map_err(Error::from)?;

        if let StatusCode::NOT_FOUND = response.status() {
            return Ok(None);
        }

        let body = response.body().await?;

        let json = serde_json::from_slice::<Value>(&body)
            .map_err(|e| Error::from(e))?;

        Ok(PokemonApiRepository::extract_first_en_flavor_text(&json))
    }

    fn extract_species(root: &Value) -> Option<String> {
        root["species"]["url"].as_str().map(|str| str.to_string())
    }

    fn extract_first_en_flavor_text(root: &Value) -> Option<String> {
        let flavor_text_entries_values = root["flavor_text_entries"].clone();
        let flavor_text_entries = serde_json::from_value::<Vec<PokemonFlavorTextEntry>>(flavor_text_entries_values).ok()?;

        flavor_text_entries.iter()
            .find(|flavor_text| flavor_text.language.name == "en")
            .map(|flavor_text| flavor_text.flavor_text.replace(|c: char| c.is_ascii_control(), " ").trim().to_string())
    }
}

#[derive(Serialize, Deserialize)]
struct PokemonFlavorTextEntry {
    flavor_text: String,
    language: PokemonLanguage,
}

#[derive(Serialize, Deserialize)]
struct PokemonLanguage {
    name: String
}

#[cfg(test)]
mod tests {
    use actix_web::test;

    use super::*;

    #[actix_rt::test]
    async fn test_get_description_with_existing_pokemon_should_return_some_string() {
        let repo = PokemonApiRepository::default();
        let result = repo.get_description("charizard").await.unwrap();

        let description = result.unwrap();

        assert!(!description.is_empty())
    }

    #[actix_rt::test]
    async fn test_get_description_with_not_existing_pokemon_should_return_none() {
        let repo = PokemonApiRepository::default();
        let result = repo.get_description("unknown_pokemon").await.unwrap();

        assert_eq!(result, None)
    }
}