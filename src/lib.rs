//src/lib.rs

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
}

// • before calling subscribe actix-web invokes the from_request method for all subscribe’s input ar-
// guments: in our case, Form::from_request;
// • Form::from_request tries to deserialise the body into FormData according to the rules of URL-
// encoding leveraging serde_urlencoded and the Deserialize implementation of FormData, automat-
// ically generated for us by #[derive(serde::Deserialize)];
// • if Form::from_request fails, a 400 BAD REQUEST is returned to the caller. If it succeeds, subscribe
// is invoked and we return a 200 OK.


