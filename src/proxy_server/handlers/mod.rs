use actix_web::{web, HttpRequest, HttpResponse, Responder}; 
use std::collections::HashMap;
use std::fs;

pub async fn verification_handler(
    form_data: web::Form<HashMap<String,String>>,
    _req: HttpRequest,
    ) -> impl Responder 
    { 

    if let Some(raw_answer) = form_data.get("answer"){
        let answer : i32 = raw_answer.parse().unwrap();
        if answer == 4 {
            return HttpResponse::Ok().body("Verification successful");
        } else {
            return HttpResponse::BadRequest().body("Incorrect answer");
        }
    }
     
    HttpResponse::Ok().body("POST to /verify-user-status")
}

pub async fn verification_page(_req: HttpRequest) -> impl Responder{
    // Read the verification page HTML
    match fs::read_to_string("src/proxy_server/static_page/verify_page.html") {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(_) => HttpResponse::InternalServerError().body("Could not load verification page")
    }
}

