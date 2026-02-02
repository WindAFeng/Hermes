use sqlx::{MySqlPool, mysql::MySqlPoolOptions};

pub struct DBPool {
    connect_url: String,
    connect_options: MySqlPoolOptions,
}

impl DBPool {
    pub fn new(connect_url: &str) -> Self {
        Self {
            connect_url: connect_url.to_string(),
            connect_options: MySqlPoolOptions::new(),
        }
    }

    pub fn max_connections(mut self, num: u32) -> Self {
        self.connect_options = self.connect_options.max_connections(num);
        self
    }

    pub fn min_connections(mut self, num: u32) -> Self {
        self.connect_options = self.connect_options.min_connections(num);
        self
    }
    
    pub async fn build(self) -> Result<MySqlPool, sqlx::Error> {
        self.connect_options.connect(&self.connect_url).await
    }
}