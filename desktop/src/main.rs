use std::borrow::Cow;
use std::collections::HashMap;
use std::fs;

use iced::advanced::graphics::text::font_system;
use iced::widget::image::Handle;
use iced::widget::{
    Column, Image, MouseArea, button, column, container, image, row, scrollable, text, text_input,
};
use iced::{Alignment, Element, Font, Length, Task, Theme, color};
use prost::Message as ProtoMessage;
use rfd::FileDialog;
use tracing::Level;

const ICON_FONT: &[u8] = include_bytes!("/usr/share/fonts/noto/NotoColorEmoji.ttf");

pub const ICFONT: Font = Font {
    family: iced::font::Family::Name("Noto Color Emoji"),
    ..Font::DEFAULT
};

fn load_nerd_font() {
    let mut font_system = font_system().write().unwrap();

    font_system.load_font(Cow::from(ICON_FONT));
}

#[derive(ProtoMessage, Clone)]
pub struct Posts {
    #[prost(repeated, message, tag = "1")]
    posts: Vec<PostDb>,
}

#[derive(ProtoMessage, Clone, PartialEq, Eq)]
pub struct PackageList {
    #[prost(repeated, string, tag = "1")]
    pub list: Vec<String>,
}

#[derive(ProtoMessage, Clone, PartialEq, Eq)]
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

#[derive(Clone, PartialEq, ProtoMessage)]
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

#[derive(Debug, Clone)]
pub struct MainUi {
    current_page: Page,
    post_data: Option<PostDb>,
    feed_posts: Vec<PostDb>,
    loading: bool,
    post_upload: Option<PostUp>,
    pacman: String,
    packages: String,
    package_list: Vec<(String, String)>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum Page {
    #[default]
    Feed,
    Detail,
    Create,
}

#[derive(Debug, Clone)]
pub enum Message {
    Pressed(PostDb),
    Loaded(Vec<PostDb>),
    FilePick,
    Router(Page),
    TitleChanged(String),
    DescriptionChanged(String),
    PacmanChanged(String),
    PackagesChanged(String),
    SubmitPackages,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Post {
    pub image: String,
    pub text: u8,
}

impl MainUi {
    pub fn new() -> (Self, Task<Message>) {
        load_nerd_font();
        (
            MainUi {
                current_page: Page::Feed,
                post_data: None,
                feed_posts: vec![],
                loading: true,
                post_upload: None,
                pacman: "".into(),
                packages: "".into(),
                package_list: vec![],
            },
            Task::perform(MainUi::fetch_post(), Message::Loaded),
        )
    }

    pub fn view(&self) -> Element<Message> {
        let post_column = match self.current_page {
            Page::Feed => {
                if !self.loading {
                    self.feed()
                } else {
                    container("loading...").into()
                }
            }
            Page::Detail => self.post_details(),
            Page::Create => self.upload_post(),
        };

        column![self.header(), post_column].into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Pressed(u) => {
                self.current_page = Page::Detail;
                self.post_data = Some(u);
            }
            Message::Loaded(posts) => {
                self.loading = false;
                self.feed_posts = posts;
            }
            Message::FilePick => {
                if let Some(file) = FileDialog::new().pick_file() {
                    if let Some(post) = self.post_upload.as_mut() {
                        post.rice_pic = fs::read(file).unwrap();
                    } else {
                        let post_upload = PostUp {
                            rice_pic: fs::read(file).unwrap(),
                            ..Default::default()
                        };
                        self.post_upload = Some(post_upload);
                    }
                }
            }
            Message::Router(page) => {
                self.current_page = page;
            }
            Message::TitleChanged(input) => {
                if let Some(post) = self.post_upload.as_mut() {
                    post.title = input;
                } else {
                    let post_upload = PostUp {
                        title: input,
                        ..Default::default()
                    };
                    self.post_upload = Some(post_upload);
                }
            }
            Message::DescriptionChanged(input) => {
                if let Some(post) = self.post_upload.as_mut() {
                    post.description = input;
                } else {
                    let post_upload = PostUp {
                        description: input,
                        ..Default::default()
                    };
                    self.post_upload = Some(post_upload);
                }
            }
            Message::PacmanChanged(input) => {
                self.pacman = input;
            }
            Message::PackagesChanged(input) => {
                self.packages = input;
            }
            Message::SubmitPackages => {
                if !self.pacman.is_empty() && !self.packages.is_empty() {
                    let install = (self.pacman.clone(), self.packages.clone());
                    self.package_list.push(install);
                }
            }
        }
    }

    pub async fn fetch_post() -> Vec<PostDb> {
        let data = match reqwest::get("http://127.0.0.1:9690/bruh").await {
            Ok(data) => match data.bytes().await {
                Ok(data) => data,
                Err(shits) => {
                    panic!("{shits}");
                }
            },
            Err(shits) => {
                panic!("{shits}");
            }
        };
        match Posts::decode(data) {
            Ok(data) => data.posts,
            Err(shits) => {
                panic!("{shits}");
            }
        }
    }

    pub fn feed(&self) -> Element<Message> {
        let post_per_row = self.feed_posts.chunks(3);
        scrollable(column(post_per_row.map(|v| {
            row(v.iter().map(|i| {
                MouseArea::new(self.body(i).padding(5))
                    .on_press(Message::Pressed(i.to_owned()))
                    .into()
            }))
            .into()
        })))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn post_details(&self) -> Element<Message> {
        let data = self.post_data.clone().unwrap();
        let image = row![
            container(
                Image::new(data.rice_pic)
                    .width(700)
                    .height(330)
                    .content_fit(iced::ContentFit::Fill),
            )
            .align_x(Alignment::Center)
            .width(Length::Fill)
        ];

        let text_content = text(data.title).size(20);

        container(column![image, text_content].spacing(10).padding(10))
            .width(Length::Fill)
            .padding(5)
            .into()
    }

    pub fn upload_post(&self) -> Element<Message> {
        let pick_button = MouseArea::new(row![
            text("Pick the ").size(20),
            text("🌾").size(20).font(ICFONT)
        ])
        .on_press(Message::FilePick);

        let image: Element<Message> = if let Some(postup) = &self.post_upload {
            if !postup.rice_pic.is_empty() {
                container(
                    image::viewer(Handle::from_bytes(postup.rice_pic.clone()))
                        .width(700)
                        .height(330)
                        .content_fit(iced::ContentFit::Contain),
                )
                .align_x(Alignment::Start)
                .into()
            } else {
                column![].into()
            }
        } else {
            column![].into()
        };

        let title_lable: Element<Message> = text("Title:").size(20).into();

        let title_input: Element<Message> = text_input(
            "Type some title...",
            &self.post_upload.clone().unwrap_or_default().title,
        )
        .on_input(Message::TitleChanged)
        .into();

        let description_lable: Element<Message> = text("Description:").size(20).into();

        let description_input: Element<Message> = text_input(
            "Type some description...",
            &self.post_upload.clone().unwrap_or_default().description,
        )
        .on_input(Message::DescriptionChanged)
        .into();

        let add_packages: Element<Message> = container(column![
            text("Add package manager, packages"),
            row![
                text_input("pacman -Syu", &self.pacman)
                    .on_input(Message::PacmanChanged)
                    .width(200),
                text_input("vim git neofetch", &self.packages).on_input(Message::PackagesChanged),
                button(row![
                    text("📦").font(ICFONT).size(20).color(color!(0x00)),
                    text("Add").size(20).color(color!(0x00))
                ])
                .on_press(Message::SubmitPackages),
            ]
            .spacing(10),
            container(column(self.package_list.iter().map(
                |(pacman, packages)| {
                    row![text(pacman).size(20).width(200), text(packages).size(20)].into()
                }
            )))
        ])
        .padding(10)
        .style(container::rounded_box)
        .into();

        container(
            column![
                pick_button,
                image,
                title_lable,
                title_input,
                description_lable,
                description_input,
                add_packages,
            ]
            .spacing(10)
            .padding(10),
        )
        .align_x(Alignment::Start)
        .width(Length::Fill)
        .padding(5)
        .into()
    }

    pub fn theme(&self) -> Theme {
        Theme::TokyoNightStorm
    }

    pub fn header(&self) -> Element<Message> {
        row![
            container(
                MouseArea::new(text("🍚").size(32).font(ICFONT))
                    .on_press(Message::Router(Page::Create))
            )
            .align_y(Alignment::Center)
            .padding(10),
            container(text("Inari").size(30))
                .align_y(Alignment::Center)
                .align_x(Alignment::Center)
                .width(iced::Length::Fill)
                .padding(10),
            container("🦀")
        ]
        .into()
    }

    #[allow(elided_named_lifetimes)]
    pub fn body<'a>(&'a self, data: &'a PostDb) -> Column<Message> {
        let image = Image::new(&data.rice_pic)
            .width(300)
            .height(150)
            .content_fit(iced::ContentFit::Fill);

        let text_content = text(&data.title).size(20);

        let post = container(column![image, text_content].spacing(10).padding(10))
            .width(320)
            .padding(5)
            .style(container::rounded_box);

        column![post]
    }
}

fn main() -> iced::Result {
    color_eyre::install().unwrap();
    tracing_subscriber::fmt()
        .with_max_level(Level::ERROR)
        .pretty()
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    iced::application("Inari", MainUi::update, MainUi::view)
        .theme(MainUi::theme)
        .run_with(MainUi::new)
}
