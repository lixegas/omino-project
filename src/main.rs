use axum::{
    routing::{get, post},
    Router,
};

use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use tokio::sync::Mutex;
use axum::http::{Method, header}; 
use env_logger;

mod database;
mod handlers;

#[tokio::main]
async fn main() {
 
    println!("SUCCHIAMI IL PINDINDOFFI BASTARDO");
    loop {
        
    env_logger::init();
    let db = Arc::new(Mutex::new(database::db::initialize_database()));


    let cors_layer = CorsLayer::new()
        .allow_origin(Any)  
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(vec![header::CONTENT_TYPE]);

        let app = Router::new()
        .route(
            "/home/:id", 
            get({let db = db.clone(); 
                          move |path |async move { handlers::visits::get_count(path, db).await }})
        )
        .route(
            "/device/:device_id",
            post({
                let db = db.clone();
                move |path| async move { handlers::visits::visit_count(path, db).await }
            }),
        )
        .layer(cors_layer);


    println!("Server in ascolto su 127.0.0.1:3000");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
}
