use std::collections::HashMap;

use iced::widget::{Column, Image, MouseArea, column, container, row, scrollable, text};
use iced::{Alignment, Element, Font, Length, Task, Theme};
use nerd_font::NerdFont;
use nerd_font::categories::Dev;
use prost::Message as ProtoMessage;

pub const NFONT: Font = Font {
    family: iced::font::Family::Name("Hack"),
    weight: iced::font::Weight::Normal,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

#[derive(ProtoMessage, Clone)]
pub struct Posts {
    #[prost(repeated, message, tag = "1")]
    posts: Vec<PostDb>,
}

#[derive(Clone, PartialEq, ProtoMessage)]
pub struct PackageList {
    #[prost(repeated, string, tag = "1")]
    pub list: Vec<String>,
}

#[derive(ProtoMessage, Clone)]
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

#[derive(Debug, Clone)]
pub struct MainUi {
    current_page: Page,
    post_data: Option<Post>,
    feed_posts: Vec<PostDb>,
    loading: bool,
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
    Pressed(Post),
    Loaded(Vec<PostDb>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Post {
    pub image: String,
    pub text: u8,
}

impl MainUi {
    //const FONT: &'static [u8] = include_bytes!("/usr/share/fonts/TTF/ZedMonoNerdFont-Regular.ttf");

    pub fn new() -> (Self, Task<Message>) {
        (
            MainUi {
                current_page: Page::Feed,
                post_data: None,
                feed_posts: vec![],
                loading: true,
            },
            Task::perform(MainUi::fetch_post(), Message::Loaded),
        )
    }

    pub fn view(&self) -> Element<Message> {
        let post_column = if self.current_page == Page::Feed {
            if !self.loading {
                self.feed()
            } else {
                container("loading...").into()
            }
        } else {
            self.post_details()
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
        }
    }

    pub async fn fetch_post() -> Vec<PostDb> {
        let data = reqwest::get("http://127.0.0.1:9690/bruh")
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();

        Posts::decode(data).unwrap().posts
    }

    pub fn feed(&self) -> Element<Message> {
        let posts: Vec<Post> = (0..31)
            .map(|v| Post {
                image: String::from("/home/walker/github/dotfiles/Wallpapers/buddha.jpg"),
                text: v,
            })
            .collect();
        let post_per_row = posts.chunks(3);
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
                Image::new(data.image)
                    .width(700)
                    .height(330)
                    .content_fit(iced::ContentFit::Fill),
            )
            .align_x(Alignment::Center)
            .width(Length::Fill)
        ];

        let text_content = text(data.text).size(20);

        container(column![image, text_content].spacing(10).padding(10))
            .width(Length::Fill)
            .padding(5)
            .into()
    }

    pub fn theme(&self) -> Theme {
        Theme::TokyoNightStorm
    }

    pub fn header(&self) -> Element<Message> {
        row![
            container(text(Dev::Android.to_string()))
                .align_y(Alignment::Center)
                .padding(10),
            container(text("Inari").font(NFONT).size(30))
                .align_y(Alignment::Center)
                .align_x(Alignment::Center)
                .width(iced::Length::Fill)
                .padding(10)
        ]
        .into()
    }

    pub fn body(&self, data: &Post) -> Column<Message> {
        let image = Image::new(&data.image)
            .width(300)
            .height(150)
            .content_fit(iced::ContentFit::Fill);

        let text_content = text(data.text).size(20);

        let post = container(column![image, text_content].spacing(10).padding(10))
            .width(320)
            .padding(5)
            .style(container::rounded_box);

        column![post]
    }
}

fn main() -> iced::Result {
    iced::application("Test", MainUi::update, MainUi::view)
        .font(NerdFont::FONT_BYTES)
        .theme(MainUi::theme)
        .run_with(MainUi::new)
}
