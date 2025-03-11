use std::{collections::HashMap, io::Cursor};

use image::{ImageFormat, ImageReader};
use prost::Message;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct PackageList {
    #[prost(repeated, string, tag = "1")]
    pub list: Vec<String>,
}

#[derive(Clone, PartialEq, Message)]
pub struct PostUp {
    #[prost(string, tag = "1")]
    pub title: String,
    #[prost(string, tag = "2")]
    pub description: String,
    #[prost(bytes, tag = "3")]
    pub rice_pic: Vec<u8>,
    #[prost(map = "string, message", tag = "4")]
    pub packages: HashMap<String, PackageList>,
    #[prost(optional, string, tag = "5")]
    pub install_script: Option<String>,
    #[prost(optional, string, tag = "6")]
    pub uninstall_script: Option<String>,
}

#[derive(Message, Serialize, Deserialize)]
pub struct Posts {
    #[prost(repeated, message, tag = "1")]
    posts: Vec<PostDb>,
}

#[derive(Serialize, Deserialize, Message)]
pub struct PostDb {
    #[prost(string, tag = "1")]
    pub title: String,
    #[prost(string, tag = "2")]
    pub description: String,
    #[prost(string, tag = "3")]
    pub rice_pic: String,
    #[prost(map = "string, message", tag = "4")]
    pub packages: HashMap<String, PackageList>,
    #[prost(optional, string, tag = "5")]
    pub install_script: Option<String>,
    #[prost(optional, string, tag = "6")]
    pub uninstall_script: Option<String>,
    #[prost(uint64, tag = "7")]
    pub downloads: u64,
    #[prost(int64, tag = "8")]
    pub votes: i64,
}

impl TryFrom<PostUp> for PostDb {
    type Error = image::ImageError;

    fn try_from(value: PostUp) -> Result<Self, Self::Error> {
        let image = ImageReader::new(Cursor::new(value.rice_pic))
            .with_guessed_format()?
            .decode()?;

        let rice_pic = format!("{}.jpeg", Uuid::new_v4());

        image.save_with_format(&rice_pic, ImageFormat::Jpeg)?;

        Ok(PostDb {
            title: value.title,
            description: value.description,
            rice_pic,
            packages: value.packages,
            install_script: value.install_script,
            uninstall_script: value.uninstall_script,
            downloads: 0,
            votes: 0,
        })
    }
}

impl From<Vec<PostDb>> for Posts {
    fn from(value: Vec<PostDb>) -> Self {
        Self { posts: value }
    }
}
