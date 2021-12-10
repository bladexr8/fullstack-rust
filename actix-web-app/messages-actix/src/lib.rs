#[macro_use]
extern crate actix_web;

use actix_web::{middleware, web, App, HttpRequest, HttpServer, Result};
use serde::Serialize;

// publicly accessible struct
// Note port is private
pub struct MessageApp {
    port: u16,
}

// implementation of MessageApp struct
impl MessageApp {

    // create new instance of MessageApp
    pub fn new(port: u16) -> Self {
        MessageApp { port }
    }

    // boot up web server
    pub fn run(&self) -> std::io::Result<()> {
        println!("Starting http server: 127.0.0.1:{}", self.port);
        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .service(index)
        })
        .bind(("127.0.0.1", self.port))?
        .workers(8)
        .run()
    }
}

// Request Handler
#[derive(Serialize)]
struct IndexResponse {
    message: String,
}

#[get("/")]
fn index(req: HttpRequest) -> Result<web::Json<IndexResponse>> {
    let hello = req
        .headers()
        .get("hello")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_else(|| "world");

    Ok(web::Json(IndexResponse {
        message: hello.to_owned(),
    }))
}