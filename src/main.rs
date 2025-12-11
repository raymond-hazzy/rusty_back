// use actix_web::{App, HttpRequest, HttpServer, Responder, Result, web};

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req
//         .match_info()
//         .get("name")
//         .unwrap_or("world")
//         .to_string();
//     format!("Hello {}!", name)
// }

// #[actix_web::main]  

// async fn main() -> std::io::Result<()> {
//     println!("Starting server at http://127.0.0.1:8000");
    
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(greet))
//             .route("/{name}", web::get().to(greet))
//     })
//     .bind("127.0.0.1:8000")?
//     .run()
//     .await
// }

//health check (first endpoint)
// use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

// async fn health_check() -> impl Responder {
//     HttpResponse::Ok()
// }

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/health_check", web::get().to(health_check))
//     })
//     .bind("127.0.0.1:8000")?
//     .run()
//     .await
// }

// ///////////////////////////....TEST

// #[cfg(test)]
// mod tests {
//     use actix_web::dev::Response;

//     use crate::health_check;

//     #[tokio::test]
//     async fn health_check_succeeds() {
//         let response = health_check().await;
//         // This requires changing the return type of `health_check`
//         // from `impl Responder` to `HttpResponse` to compile
//         // You also need to import it with `use actix_web::HttpResponse`!
//         assert!(response.status().is_success())
//     }
// }

use::rusty_back::run;

//! src/main.rs
use std::net::TcpListener;
use rusty_back::startup::run;
use rusty_back::config::get_configuration;

#[tokio::main]

async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}

















