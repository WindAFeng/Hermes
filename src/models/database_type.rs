pub enum DatabaseType{
    Redis,
    MySQL,
    MongoDB,
    PostgreSQL,
}
impl DatabaseType {
    pub fn to_string(&self) -> String {
        match self { 
            DatabaseType::Redis => String::from("redis"),
            DatabaseType::MySQL => String::from("mysql"),
            DatabaseType::MongoDB => String::from("mongodb"),
            DatabaseType::PostgreSQL => String::from("postgresql"),
        }
    }
}