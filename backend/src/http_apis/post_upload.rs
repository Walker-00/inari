use actix_protobuf::ProtoBuf;

use crate::structures::{post::Post, static_vars::DB};

pub fn post_upload(post: ProtoBuf<Post>) {}
