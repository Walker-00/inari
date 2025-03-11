use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpResponse, Responder};

use crate::structures::{
    post::{PostDb, PostUp},
    static_vars::DB,
};

pub async fn post_upload(post: ProtoBuf<PostUp>) -> impl Responder {
    let postdb: PostDb = match post.0.try_into() {
        Ok(postdb) => postdb,
        Err(shits) => {
            return HttpResponse::BadRequest().protobuf(shits.to_string());
        }
    };
    match DB.create::<Option<PostDb>>("rices").content(postdb).await {
        Ok(Some(_)) => HttpResponse::Ok().protobuf("ok".to_string()),
        Ok(None) => HttpResponse::InternalServerError()
            .protobuf("shits just happened in creating a post".to_string()),
        Err(shits) => HttpResponse::InternalServerError().protobuf(shits.to_string()),
    }
}
