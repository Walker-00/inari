use std::collections::HashMap;

use prost::Message;

#[derive(Clone, PartialEq, Message)]
pub struct PackageList {
    #[prost(repeated, string, tag = "1")]
    pub list: Vec<String>,
}

#[derive(Clone, PartialEq, Message)]
pub struct Post {
    #[prost(string, tag = "1")]
    pub title: String,
    #[prost(string, tag = "2")]
    pub description: String,
    #[prost(bytes, tag = "3")]
    pub rice_pics: Vec<u8>,
    #[prost(map = "string, message", tag = "4")]
    pub packages: HashMap<String, PackageList>,
    #[prost(optional, string, tag = "5")]
    pub install_script: Option<String>,
    #[prost(optional, string, tag = "6")]
    pub uninstall_script: Option<String>,
}
