mod handlers;

use actix_web::{App, HttpServer,web};
use handlers::{verification_handler,verification_page,proxy_handler};
use std::{io,env};
use chrono::Local;


pub struct ProxyConfig {
    pub backend_url:String,
}
impl Default for ProxyConfig {
    fn default() -> Self {
        ProxyConfig{
            backend_url: "127.0.0.1:4000".to_string(),
        }
    }
}

#[actix_web::main]
pub async fn run_proxy_server()->io::Result<()>{

    let host: String = env::var("HOST").unwrap_or_else(|_|{"127.0.0.1".to_string()});
    let port: String = env::var("PORT").unwrap_or_else(|_|{8080.to_string()});
    let curr_time = Local::now().format("[%Y-%m-%d](%H:%M:%S)");

    println!("{} Server running http://{}:{}/",curr_time,host,port);
    let proxy_config = web::Data::new(ProxyConfig::default());
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("")
                .app_data(proxy_config.clone())
                .route("/",
                    web::get().to(proxy_handler))
                .route("/",
                    web::post().to(proxy_handler))
                .route("/verify-user-status",
                    web::post().to(verification_handler)) 
                .route("/verification-page",
                    web::get().to(verification_page)) 
            )
    }).bind((host,port.parse().unwrap()))?
      .run()
      .await
}
