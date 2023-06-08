#![allow(non_snake_case, non_camel_case_types, dead_code)]

use server::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run().await
}
