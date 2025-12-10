use actix_web::{App, Error, HttpResponse, HttpServer, web};
use actix_web::dev::Server;
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Let's start simple: we always return a 200 OK

async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        app::new()
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?//use pre-bound listener
    .run();
    
    Ok(server)  // Don't await here!
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}//hmm


