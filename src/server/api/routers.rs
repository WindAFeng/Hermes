use actix_web::web;
use crate::server::api::handle_request::{database_state, health};
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health))
            .service(
                web::scope("/database")
                    .route("/state", web::get().to(database_state))
            )
    );
}