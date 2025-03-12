use iced::widget::{Column, Image, MouseArea, column, container, row, scrollable, text};
use iced::{Alignment, Element, Font, Length, Theme};
use nerd_font::NerdFont;
use nerd_font::categories::Dev;

pub const NFONT: Font = Font {
    family: iced::font::Family::Name("Hack"),
    weight: iced::font::Weight::Normal,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct MainUi {
    pub current_page: Page,
    pub post_data: Option<Post>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum Page {
    #[default]
    Feed,
    Detail,
    Create,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Message {
    Pressed(Post),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Post {
    pub image: String,
    pub text: u8,
}

impl MainUi {
    //const FONT: &'static [u8] = include_bytes!("/usr/share/fonts/TTF/ZedMonoNerdFont-Regular.ttf");

    pub fn view(&self) -> Element<Message> {
        let post_column = if self.current_page == Page::Feed {
            self.feed()
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
        }
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
        .run()
}
