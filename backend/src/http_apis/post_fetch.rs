use actix_protobuf::ProtoBufResponseBuilder;
use actix_web::{HttpResponse, Responder, get};

use crate::structures::{
    post::{PostDb, Posts},
    static_vars::DB,
};

#[get("/fetch/rice")]
pub async fn post_fetch() -> impl Responder {
    let query = "SELECT * FROM rices ORDER BY date DESC;";

    let resp = DB.query(query).await.unwrap();

    let mut resp = match resp.check() {
        Ok(resp) => resp,
        Err(shits) => return HttpResponse::InternalServerError().protobuf(shits.to_string()),
    };

    let posts: Posts = resp.take::<Vec<PostDb>>(0).unwrap().into();

    HttpResponse::Ok().protobuf(posts)
}
