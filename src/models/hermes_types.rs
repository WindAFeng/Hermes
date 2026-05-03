use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum HermesTypes {
    String,
    Int,
    UInt,
    Float,
    Bool,
    HashMap,
    List,
    None
}
