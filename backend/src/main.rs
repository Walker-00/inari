use std::io;

use actix_web::{App, HttpServer};
use prost::Message;

#[derive(Clone, PartialEq, Message)]
pub struct Idk {
    #[prost(string)]
    pub name: String,
    #[prost(uint32)]
    pub age: u32,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new())
        .bind("127.0.0.1:9690")?
        .run()
        .await
}
