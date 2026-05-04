use crate::errors::HermesError;
use actix_web::{App, HttpServer};
use crate::utils::{config::get_config};
use crate::server::api::routers;
pub struct WebApp{
    host: String,
    port: u16,
}
impl WebApp{
    pub fn new() -> Self{
        let config = &get_config();
        Self{
            host: config.web.host.clone(),
            port: config.web.port,
        }
    }
    pub async fn start(&self) -> Result<(), HermesError>{
        let web = HttpServer::new(|| {
            App::new().configure(routers::config)
        })
            .bind((self.host.clone(), self.port))?;
        println!("Web server listening on:");
        println!(" - http://{}:{}", self.host, self.port);
        web.run().await?;
        Ok(())
    }
}
