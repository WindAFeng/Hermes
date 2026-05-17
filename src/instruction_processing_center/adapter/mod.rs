pub mod database_adapt;
mod mysql_adapter;
mod redis_adapter;
mod postgresql_adapter;
mod mongodb_adapter;
pub type MongoDBAdapter = mongodb_adapter::MongoDBAdapter;
pub type MySQLAdapter = mysql_adapter::MySQLAdapter;
pub type RedisAdapter = redis_adapter::RedisAdapter;
pub type PostgreSQLAdapter = postgresql_adapter::PostgreSQLAdapter;
