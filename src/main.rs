//use std::net::SocketAddr;
//
//use axum::{response::Html, Router};
//#[tokio::main]
//async fn main() {
//    let routes_hello = Router::new().route(
//        "/hello",
//        get(|| async { Html("Hello <strong>World!!!</strong")}),
//    );
//    let addr = SocketAddr::from(([127,0,0,0,1], 8080));
//    println!("Listening on {addr}\n");
//    axum::Server::bind(&addr)
//    .serve(routes_hello.into_make_device();)
//    
//    .unwrap();

use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn hello_world() -> &'static str {
    "Hello world!"
}

#[tokio::main] 
async fn main() {
    let router = Router::new().route("/", get(hello_world));

    let addr = SocketAddr::from(([127,0,0,1], 8000));
    let tcp = TcpListener::bind(&addr).await.unwrap();

    axum::serve(tcp, router).await.unwrap();
}