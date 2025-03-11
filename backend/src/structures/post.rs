use std::{collections::HashMap, io::Cursor};

use chrono::Utc;
use image::ImageReader;
use prost::Message;
use serde::{Deserialize, Serialize};
use surrealdb::Datetime;

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

#[derive(Serialize, Deserialize)]
pub struct PostDb {
    pub date: Datetime,
    pub title: String,
    pub description: String,
    pub rice_pic: String,
    pub packages: HashMap<String, PackageList>,
    pub install_script: Option<String>,
    pub uninstall_script: Option<String>,
    pub downloads: u64,
    pub votes: i64,
}

impl TryFrom<PostUp> for PostDb {
    type Error = image::ImageError;

    fn try_from(value: PostUp) -> Result<Self, Self::Error> {
        let image = ImageReader::new(Cursor::new(value.rice_pic))
            .with_guessed_format()?
            .decode()?;

        image.save_with_format("", format);
        Ok(PostDb {
            date: Utc::now().into(),
            title: value.title,
            description: value.description,
            rice_pic: "".into(),
            packages: value.packages,
            install_script: value.install_script,
            uninstall_script: value.uninstall_script,
            downloads: 0,
            votes: 0,
        })
    }
}
