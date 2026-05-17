use std::collections::HashMap;
use actix_web::HttpResponse;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::api_model::{health::Health, database_state::DatabaseState};
use crate::utils::time::timestamp;

pub async fn health() -> Result<HttpResponse, HermesError> {
    let response = Health {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: timestamp().to_string(),
    };
    Ok(HttpResponse::Ok().json(response))
}

pub async fn database_state() -> Result<HttpResponse, HermesError> {
    let response = DatabaseState {
        status: HashMap::new(),
        timestamp: timestamp().to_string(),
    };
    Ok(HttpResponse::Ok().json(response))
}
