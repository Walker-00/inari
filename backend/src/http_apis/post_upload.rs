use actix_protobuf::ProtoBuf;
use surrealdb::Datetime;

use crate::structures::{post::PostUp, static_vars::DB};

pub fn post_upload(post: ProtoBuf<PostUp>) {}
