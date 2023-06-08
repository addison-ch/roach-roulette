#![allow(non_snake_case, non_camel_case_types, dead_code)]

use std::net::TcpListener;

use server::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    run(listener)?.await
}
