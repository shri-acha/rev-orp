mod handlers;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use std::{io,env};
use chrono::Local;
use handlers::mock_service_handler;


pub async fn run_mock_server()->io::Result<()>{

    let host: String = env::var("HOST").unwrap_or_else(|_|{"127.0.0.1".to_string()});
    let port: String = env::var("PORT").unwrap_or_else(|_|{1234.to_string()});
    let curr_time = Local::now().format("[%Y-%m-%d](%H:%M:%S)");

    println!("{} Mock - Server running http://{}:{}/",curr_time,host,port);
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("")
                .route("/",
                    web::get().to(mock_service_handler)),
            )
    }).bind((host,port.parse().unwrap()))?
      .run()
      .await
}
