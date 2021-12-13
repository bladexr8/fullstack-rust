#[macro_use]
extern crate actix_web;

// web server modules from Actix
use actix_web::{middleware, web, App, HttpRequest, HttpServer, Result};
// JSON Serialise/De-serialise
use serde::Serialize;

// facilitate mutating properties with an immutable struct
use std::cell::Cell;

// tools to work with usize atomically and thread-safe
use std::sync::atomic::{AtomicUsize, Ordering};
// tools to safely share and mutate non-atomic things across multiple threads
use std::sync::{Arc, Mutex};

// track count of workers
static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

// publicly accessible struct
// Note port is private
pub struct MessageApp {
    port: u16,
}

// application state (each worker gets own instance)
struct AppState {
    server_id: usize,
    request_count: Cell<usize>,
    messages: Arc<Mutex<Vec<String>>>,
}

// implementation of MessageApp struct
impl MessageApp {

    // create new instance of MessageApp
    pub fn new(port: u16) -> Self {
        MessageApp { port }
    }

    // boot up web server
    pub fn run(&self) -> std::io::Result<()> {
        // share messages across workers
        let messages = Arc::new(Mutex::new(vec![]));
        println!("Starting http server: 127.0.0.1:{}", self.port);
        HttpServer::new(move || {
            App::new()
                // add application state
                .data(AppState {
                    server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
                    request_count: Cell::new(0),
                    messages: messages.clone(),
                })
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
    server_id: usize,
    request_count: usize,
    messages: Vec<String>,
}

#[get("/")]
fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {

    // update # requests for this server
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    // access data inside the Mutex
    let ms = state.messages.lock().unwrap();
    

    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        messages: ms.clone(),
    }))
}