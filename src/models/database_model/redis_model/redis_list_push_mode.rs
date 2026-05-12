use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone, Default)]
pub enum RedisListPushMode {
    #[default]
    Left,
    Right,
}
