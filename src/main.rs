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

use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}