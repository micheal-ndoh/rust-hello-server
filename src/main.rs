

use axum::{
    routing::{get, post}, Router,
};


#[tokio::main]
async fn main() {
    
    tracing_subscriber::fmt::init();

    let app = Router::new()
       
        .route("/", get(root))
        .route("/hello", get(hello));
 
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn hello() -> (&'static str) {
    "Hi I am Micheal!"
}
