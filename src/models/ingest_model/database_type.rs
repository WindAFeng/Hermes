use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum DatabaseType {
   #[serde(rename = "MySQL")]
   MySql,
   #[serde(rename = "MongoDB")]
   MongoDB,
   #[serde(rename = "Redis")]
   Redis,
   #[serde(rename = "PostgreSQL")]
   PostgreSQL,
}