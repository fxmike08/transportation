use serde::{Deserialize, Serialize};
use transportation_api::models::Errors as ModelError;

pub use self::ServiceError::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum ServiceError {
    DataBaseError,
    NotFound,
    InternalError,
}

impl From<ServiceError> for ModelError {
    fn from(s: ServiceError) -> ModelError {
        match s {
            NotFound => make_err("001", "The specified record does not exist."),
            DataBaseError => make_err(
                "100",
                "An error occurred in the underlying storage database.",
            ),
            InternalError => make_err("500", "An internal error occurred."),
        }
    }
}

fn make_err(id: &str, message: &str) -> ModelError {
    ModelError {
        request_id: None,
        message: message.to_string(),
        origin: "Transportation".to_string(),
        error_id: id.to_string(),
        variables: None,
    }
}
