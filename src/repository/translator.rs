use std::collections::HashMap;

use actix_web::client::{Client, Connector, SendRequestError};
use openssl::ssl::{SslConnector, SslMethod};
use serde_json::Value;

use crate::errors::RestError;

pub struct TranslatorApiRepository {
    client: Client
}

impl TranslatorApiRepository {
    pub fn default() -> TranslatorApiRepository {
        let builder = SslConnector::builder(SslMethod::tls())
            .unwrap();

        let client = Client::build()
            .connector(Connector::new().ssl(builder.build()).finish())
            .finish();

        TranslatorApiRepository { client }
    }

    pub async fn get_shakespearean_translation(&self, text: &str) -> Result<String, RestError> {
        let mut form = HashMap::with_capacity(1);
        form.insert("text", text);

        let mut response = self.client.post("https://api.funtranslations.com/translate/shakespeare.json")
            .send_form(&form)
            .await
            .map_err(|e: SendRequestError| RestError::UpstreamError(e.to_string()))?;

        let body = response.body().await
            .map_err(|e| RestError::UpstreamError(e.to_string()))?;

        let json = serde_json::from_slice::<Value>(&body)
            .map_err(|e| RestError::UpstreamError(e.to_string()))?;

        json["contents"]["translated"].as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| RestError::UpstreamError("Translation parsing error!".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::translator::TranslatorApiRepository;

    #[actix_rt::test]
    async fn test_translate_text_should_return_ok_with_string() {
        let repo = TranslatorApiRepository::default();

        let result = repo.get_shakespearean_translation("Spits fire that is hot enough to melt boulders. Known to cause forest fires unintentionally.").await.unwrap();

        assert!(!result.is_empty())
    }
}