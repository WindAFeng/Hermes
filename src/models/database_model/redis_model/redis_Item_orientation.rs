pub enum RedisItemOrientation {
    Before,
    After,
}
impl RedisItemOrientation {
    pub fn to_string(&self) -> String {
        match &self {
            RedisItemOrientation::Before => String::from("BEFORE"),
            RedisItemOrientation::After => String::from("AFTER"),
        }
    }
}