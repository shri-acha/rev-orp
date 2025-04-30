use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse, Responder, ResponseError}; 
use std::collections::HashMap;


const VERIFICATION_PAGE: &str = r#"
        <html>
            <body>
                <h1>Human Verification</h1>
                <form action="/verify" method="post">
                    <label>What is 2 + 2?</label>
                    <input name="answer" />
                    <button type="submit">Submit</button>
                </form>
            </body>
        </html>
    "#;



pub async fn verification_handler<R>(
    form_data: web::Form<HashMap<String,String>>,
    req:HttpRequest,
    ) -> impl Responder 
    { 

    if let Some(raw_answer) = form_data.get("answer"){
        let answer : i32 = raw_answer.parse().unwrap();
        if answer == 4 {
        }
    }
     
    HttpResponse::Ok().body("POST to /verify-user-status")
}

pub async fn verification_page(req: HttpRequest) -> impl Responder{
    let response = HttpResponse::new(StatusCode::from_u16(200).unwrap());
    
    response
}

