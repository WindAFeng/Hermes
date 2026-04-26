use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HermesTypes {
    String,
    Integer,
    UnsignedInteger,
    Float,
    Bool,
    HashMap,
    List,
    None
}
