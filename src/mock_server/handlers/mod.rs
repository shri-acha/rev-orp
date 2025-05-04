use actix_web::{HttpResponse,Responder};

pub async fn mock_service_handler()->impl Responder{ 
        HttpResponse::Ok().body("Hello World!") 
    }
