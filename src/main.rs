mod proxy_server;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Modified to run directly on port 8080
    let host = "127.0.0.1";
    let port = 8080;

    HttpServer::new(move || {
        // We'll still use the proxy_config structure but we're not actually proxying now
        let proxy_config = web::Data::new(proxy_server::ProxyConfig {
            backend_url: format!("http://{}:{}", host, port),
        });

        App::new()
            .app_data(proxy_config.clone())
            // Add the verification page route
            .route("/verification-page", web::get().to(proxy_server::handlers::verification_page))
            // Add the verification handler route for form submission
            .route("/verify", web::post().to(proxy_server::handlers::verification_handler))
            // Existing proxy handler for all other routes
            .default_service(web::route().to(proxy_server::handlers::proxy_handler))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
