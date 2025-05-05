use actix_web::{HttpRequest, HttpResponse, Responder};
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
struct testStruct {
    test_field_1:i32,
    test_field_2:i32,
}

pub async fn mock_service_handler(req:HttpRequest)->impl Responder{ 
    let test_data = testStruct{
        test_field_1 : 01,
        test_field_2 : 02,
    };
    let curr_time = Local::now().format("(%H:%M:%S)");
    println!("{}[REV-ORP] Incoming request to mock - server:{:?}",curr_time,req);
        HttpResponse::Ok().json(test_data) 
    }
