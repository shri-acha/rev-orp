use actix_web::http::header::{self, LOCATION};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use awc::cookie::Cookie;
use std::fs::File;
use std::collections::HashMap;
use std::io::Read;
use chrono::Local; 
use awc::Client;
use super::ProxyConfig;




//I'm unsure whether if to let the verification page store the request that is sent
//by the user originally to the backend or if let the backend do its work.
pub async fn proxy_handler (
    req:HttpRequest,
    config: web::Data<ProxyConfig>
    ) -> impl Responder 
    { 
    let curr_time = Local::now().format("(%H:%M:%S)");
    println!("{}[REV-ORP] Incoming request to server:{:?}",curr_time,req);

    let cookie_verification_status = req.cookie("user_verified");
    if let Some(cookie_verification_status) = cookie_verification_status {
        if cookie_verification_status.value() == "true"{
            proxy_to_backend(&req,actix_web::web::Bytes::new(),&config).await
        }else {
            HttpResponse::Found()
                .append_header((LOCATION,"/verification-page"))
                .finish()
        }
    }else {
            HttpResponse::Found()
                .append_header((LOCATION,"/verification-page"))
                .finish()
        }


}


async fn proxy_to_backend(req: &HttpRequest,body: web::Bytes,config: &ProxyConfig)-> HttpResponse{

    let awc_client = Client::default();

    let path = req.uri().path_and_query()
        .map(|p| { p.as_str()})
        .unwrap_or("/");

    let backend_url = format!("{}{}",config.backend_url,path);

    let mut backend_req = awc_client.request(req.method().clone(), backend_url);
    
    for ( header_name, header_value) in req.headers().iter()
        .filter(|(n,_)|{*n != "host"})
        {
        backend_req = backend_req.append_header((header_name.clone(), header_value.clone()));
    }
    match backend_req.send_body(body).await {
        Ok(mut response)=>{
            let mut client_resp = HttpResponse::build(response.status());
            for (header_name,header_value) in response.headers().iter().filter(|(n,_)|{*n != "connection" || *n != "content-length" }) {
                client_resp.append_header((header_name.clone(),header_value.clone()));
            }
        match response.body().await {
            Ok(bytes) => {
                client_resp.body(bytes);
            }
            Err(_)=>{
                HttpResponse::InternalServerError().body("Failed to reach backend!");
            }


        }
        }
        Err(err) => {
            println!("Error in forwarding request to backend! {}",err);
        }
    }
    HttpResponse::Ok().body("FALLBACK")
}



pub async fn verification_page(_req: HttpRequest) -> impl Responder{
    //Static Site Handler
    let mut verification_page:String = String::new(); 
    File::open("src/proxy_server/static_page/verify_page.html").unwrap().read_to_string(&mut verification_page).unwrap();
    let resp = HttpResponse::Ok().body(verification_page);
    resp
}
pub async fn verification_handler(form_data: web::Form<HashMap<String,String>>,req: HttpRequest)->impl Responder{
    // Handles the logic for captcha validation and post-validation.
    if let Some(raw_answer) = form_data.get("answer"){
        let answer : i32 = raw_answer.parse().unwrap_or_else(|_|{
            -1
        });
        if  4 == answer {
            HttpResponse::Found()
                .insert_header((LOCATION,"/"))
                .cookie(Cookie::build("user_verified","true").finish())
                .finish()
        }else {

            let mut resp =  HttpResponse::Forbidden();
                resp.cookie(Cookie::build("user_verified","false").finish());
                resp.body("Captcha Failed!");
                resp.finish()
        }
    }else {
            let mut resp =  HttpResponse::Forbidden();
                resp.cookie(Cookie::build("user_verified","false").finish());
                resp.body("Captcha Failed!");
                resp.finish()
        }

}

