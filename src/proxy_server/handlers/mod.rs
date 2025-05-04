use actix_web::http::header::{self, LOCATION};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use awc::cookie::Cookie;
use std::collections::HashMap;
use chrono::Local; 
use awc::Client;
use super::ProxyConfig;
use rand::Rng;
use std::sync::RwLock;
use once_cell::sync::Lazy;
use uuid::Uuid;
use bytes::BytesMut;
use futures::StreamExt;



//=================YHA K VAIRAKO XA TIMI LAI JATI IDEA XA
//=================MALAI NE TATE NAI
//=================(MALAI NE THA XAINA K VAYO JUST ASK CLAUDE K GARISH VANERA 💅)



// session ID hola sayed captcha ko lagi
// session ID ko lagi chai hashmap ma store garne
static CAPTCHA_CHALLENGES: Lazy<RwLock<HashMap<String, i32>>> = Lazy::new(|| RwLock::new(HashMap::new()));

pub async fn proxy_handler(
    req: HttpRequest,
    mut body: web::Payload,
    config: web::Data<ProxyConfig>
) -> impl Responder { 
    let curr_time = Local::now().format("(%H:%M:%S)");
    println!("{}[REV-ORP] Incoming request to server:{:?}", curr_time, req);

    let cookie_verification_status = req.cookie("user_verified");
    
    // Correctly handle payload conversion using StreamExt::next()
    let mut body_bytes = BytesMut::new();
    while let Some(chunk) = body.next().await {
        match chunk {
            Ok(chunk) => {
                body_bytes.extend_from_slice(&chunk);
            }
            Err(e) => {
                println!("Error reading request body chunk: {}", e);
                return HttpResponse::BadRequest().body("Failed to read request body");
            }
        }
    }
    
    if let Some(cookie_verification_status) = cookie_verification_status {
        if cookie_verification_status.value() == "true" {
            // Instead of proxying, serve direct content
            HttpResponse::Ok()
                .content_type("text/html")
                .body("<html><body><h1>Welcome to the Protected Content!</h1><p>You have successfully verified and can now access the content.</p></body></html>")
        } else {
            HttpResponse::Found()
                .append_header((LOCATION, "/verification-page"))
                .finish()
        }
    } else {
        HttpResponse::Found()
            .append_header((LOCATION, "/verification-page"))
            .finish()
    }
}


// backend request ko lagi 
async fn proxy_to_backend(req: &HttpRequest, body: web::Bytes, config: &ProxyConfig) -> HttpResponse {
    let awc_client = Client::default();

    let path = req.uri().path_and_query()
        .map(|p| p.as_str())
        .unwrap_or("/");
    let backend_url = format!("{}{}", config.backend_url, path);

    let mut backend_req = awc_client.request(req.method().clone(), backend_url);
    
    // Copy headers from original request to backend request
    for (header_name, header_value) in req.headers().iter()
        .filter(|(n, _)| *n != header::HOST)
    {
        backend_req = backend_req.append_header((header_name.clone(), header_value.clone()));
    }

   
    match backend_req.send_body(body).await {
        Ok(mut response) => {
           
            let mut client_resp = HttpResponse::build(response.status());
            
            // Fixed header filter logic with correct boolean operator (AND instead of OR)
            for (header_name, header_value) in response.headers().iter()
                .filter(|(n, _)| *n != header::CONNECTION && *n != header::CONTENT_LENGTH) {
                client_resp.append_header((header_name.clone(), header_value.clone()));
            }
            
            // Return the actual backend response body
            match response.body().await {
                Ok(bytes) => {
                    return client_resp.body(bytes);
                }
                Err(e) => {
                    println!("Error reading backend response body: {}", e);
                    return HttpResponse::InternalServerError().body("Failed to read backend response");
                }
            }
        }
        Err(err) => {
            println!("Error in forwarding request to backend: {}", err);
            return HttpResponse::BadGateway().body(format!("Failed to connect to backend: {}", err));
        }
    }
}



pub async fn verification_page(_req: HttpRequest) -> impl Responder {
    //maths genius ley matrai solve garna sakxa
    let mut rng = rand::thread_rng();
    let num1 = rng.gen_range(1..20);
    let num2 = rng.gen_range(1..20);
    let answer = num1 + num2;
    
    // Generate a session ID
    let session_id = Uuid::new_v4().to_string();
    
    // Store the answer with session ID
    CAPTCHA_CHALLENGES.write().unwrap().insert(session_id.clone(), answer);
    
    // Load the verification HTML template
    match std::fs::read_to_string("src/proxy_server/static_page/verify_page.html") {
        Ok(page) => {
            // Replace placeholder values with our dynamic content
            let page = page
                .replace("<span id=\"num1\">2</span>", &format!("<span id=\"num1\">{}</span>", num1))
                .replace("<span id=\"num2\">2</span>", &format!("<span id=\"num2\">{}</span>", num2))
                .replace("value=\"{{SESSION_ID}}\"", &format!("value=\"{}\"", session_id));
            
            HttpResponse::Ok().content_type("text/html").body(page)
        },
        Err(e) => {
            println!("Failed to load verification page: {}", e);
            HttpResponse::InternalServerError().body("Failed to load verification page")
        }
    }
}

pub async fn verification_handler(form_data: web::Form<HashMap<String, String>>, req: HttpRequest) -> impl Responder {
    // Get session ID from form data
    let session_id = match form_data.get("session_id") {
        Some(id) => id,
        None => return HttpResponse::BadRequest().body("Missing session ID")
    };
    
    // Get user answer from form data
    let user_answer: i32 = match form_data.get("answer").and_then(|s| s.parse().ok()) {
        Some(num) => num,
        None => return HttpResponse::BadRequest().body("Invalid answer format")
    };
    
    // Check if we have a stored challenge for this session
    let expected_answer = {
        let challenges = CAPTCHA_CHALLENGES.read().unwrap();
        match challenges.get(session_id) {
            Some(&answer) => answer,
            None => return HttpResponse::BadRequest().body("Invalid or expired session")
        }
    };
    
    // Validate the answer
    if user_answer == expected_answer {
        // Remove the used challenge to prevent replay attacks
        CAPTCHA_CHALLENGES.write().unwrap().remove(session_id);
        
        // Set verification cookie with appropriate security settings
        let cookie = Cookie::build("user_verified", "true")
            .path("/")
            .http_only(true)
            .secure(req.connection_info().scheme() == "https") // Only set secure flag if HTTPS
            .max_age(time::Duration::hours(1))
            .finish();
        
        return HttpResponse::Found()
            .append_header((LOCATION, "/"))
            .cookie(cookie)
            .finish();
    } else {
        return HttpResponse::Ok().content_type("text/html").body("
            <h2>Incorrect answer</h2>
            <p>The answer you provided was incorrect. Please try again.</p>
            <p><a href='/verification-page'>Go back to verification</a></p>
        ");
    }
}

