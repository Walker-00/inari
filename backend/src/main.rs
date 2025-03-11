use std::{fs, io, path::Path};

use actix_web::{App, HttpServer};
use prost::Message;
use structures::static_vars::{DATA_PATH, DB};

mod http_apis;
mod structures;

#[derive(Clone, PartialEq, Message)]
pub struct Idk {
    #[prost(string)]
    pub name: String,
    #[prost(uint32)]
    pub age: u32,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    DB.use_ns("namespace").use_db("inari").await.unwrap();

    let path = Path::new(&*DATA_PATH);

    if !path.exists() {
        fs::create_dir_all(&*DATA_PATH);
    } else if !path.is_dir() {
        panic!("The Data Path: {} is not a dri", &*DATA_PATH);
    }

    HttpServer::new(|| App::new())
        .bind("127.0.0.1:9690")?
        .run()
        .await
}
