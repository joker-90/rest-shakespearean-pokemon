use actix_web::{error, http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};
use serde::export::fmt;
use serde::export::Formatter;

#[derive(Debug)]
pub enum RestError {
    PokemonNotFound(String),
    UpstreamError(String),
}

impl fmt::Display for RestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RestError::PokemonNotFound(name) => write!(f, "Pokemon with name {} not found!", name),
            RestError::UpstreamError(cause) => write!(f, "Something went wrong: {}", cause),
        }
    }
}

impl error::ResponseError for RestError {
    fn status_code(&self) -> StatusCode {
        match self {
            RestError::PokemonNotFound(_) => StatusCode::NOT_FOUND,
            RestError::UpstreamError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(ErrorResponseBody { message: self.to_string() })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponseBody {
    message: String
}