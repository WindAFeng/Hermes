use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct DatabaseState {
    pub(crate) timestamp: String,
    pub(crate) status: HashMap<String, bool>,
}
