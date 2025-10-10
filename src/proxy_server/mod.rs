mod handlers;

use actix_web::{App, HttpServer,web};
use handlers::{verification_handler,verification_page,proxy_handler,captcha_handler};
use std::{io,env};
use chrono::Local;
use actix_session::{Session,SessionMiddleware,storage::CookieSessionStore};
use uuid::Uuid;
use actix_web::cookie::Key;


pub struct ProxyConfig {
    pub backend_url:String,
}
impl Default for ProxyConfig {
    fn default() -> Self {
        ProxyConfig{
            backend_url:"http://127.0.0.1:1234".to_string(),
        }
    }
}

pub async fn run_proxy_server()->io::Result<()>{

    let host: String = env::var("HOST").unwrap_or_else(|_|{"127.0.0.1".to_string()});
    let port: String = env::var("PORT").unwrap_or_else(|_|{8080.to_string()});
    let curr_time = Local::now().format("[%Y-%m-%d](%H:%M:%S)");

    println!("{} Proxy - Server running http://{}:{}/",curr_time,host,port);

    let proxy_config = web::Data::new(ProxyConfig::default());
    let key = Key::generate();

    HttpServer::new(move || {
    App::new()
        .app_data(proxy_config.clone())
        .wrap(
            SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                    .cookie_secure(false)
                    .build()
        )
        .route("/verify-user-status", web::post().to(verification_handler))
        .route("/verification-page", web::get().to(verification_page))
        .route("/captcha/generate", web::get().to(captcha_handler))
        // Catch-all for GET and POST at any other path
        .route("/{tail:.*}", web::get().to(proxy_handler))
        .route("/{tail:.*}", web::post().to(proxy_handler))
}).bind((host,port.parse().unwrap()))?
      .run()
      .await
}
