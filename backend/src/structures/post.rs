use prost::Message;

#[derive(Clone, PartialEq, Message)]
pub struct Post {
    #[prost(string)]
    pub title: String,
    #[prost(string)]
    pub description: String,
    #[prost(optional, string)]
    pub install_script: Option<String>,
    #[prost(optional, string)]
    pub uninstall_script: Option<String>,
    #[prost(bytes)]
    pub rice_pics: Vec<u8>,
}
