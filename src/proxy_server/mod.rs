mod handlers;

use actix_web::{App, HttpServer,web};
use handlers::{verification_handler,verification_page};
use std::io;
#[actix_web::main]
pub async fn run_proxy_server()->io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("")
                .route("/verify-user-status",
                    web::post().to(verification_handler)) 
                .route("/verification-page",
                    web::get().to(verification_page)) 
            )
    }).bind(("127.0.0.1",8080))?
      .run()
      .await
}
