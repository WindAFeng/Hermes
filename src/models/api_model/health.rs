use serde::Serialize;

#[derive(Serialize)]
pub struct Health {
    pub(crate) status: String,
    pub(crate) version: String,
    pub(crate) timestamp: String,
}