use configparser::ini::Ini;
use iced::alignment;
use iced::theme;
use iced::theme::PickList;
use iced::widget::button;
use iced::widget::pick_list;
use iced::widget::scrollable::Scrollable;
use iced::widget::{
    checkbox, column, container, horizontal_space, image, radio, row, scrollable, slider, text,
    text_input, toggler, vertical_space,
};
use iced::widget::{Button, Column, Container, Slider};
use iced::window::icon;
use iced::window::Icon;
use iced::Alignment;
use iced::{Color, Element, Length, Renderer, Sandbox, Settings};
use std::env;
use std::fs;
use std::path;
use std::path::Path;
fn main() -> iced::Result {
    Clipboard::run(Settings {
        id: None,
        window: iced::window::Settings {
            size: (500, 500),
            position: iced::window::Position::Centered,
            min_size: Some((500, 500)),
            max_size: Some((500, 500)),
            visible: true,
            resizable: false,
            decorations: false,
            transparent: false,
            always_on_top: true,
            icon: None,
        },
        flags: (),
        default_font: None,
        default_text_size: 20,
        text_multithreading: false,
        antialiasing: true,
        exit_on_close_request: true,
        try_opengles_first: false,
    })
}
#[derive(Debug, Clone)]
enum Message {
    Increase,
    Decrease,
}
struct Clipboard {
    temp: u8,
}
impl Sandbox for Clipboard {
    type Message = Message;

    fn new() -> Self {
        if !Path::new("config.ini").exists() {
            let mut config = Ini::new();
            config.set("Temp", "temp", Some(String::from("20")));
            config.write("config.ini");
        }
        let mut config = Ini::new();
        config.load("config.ini");
        let tmp = config.get("Temp", "temp").unwrap();
        let tmp: u8 = tmp.trim().parse().unwrap();
        Clipboard { temp: tmp }
    }

    fn title(&self) -> String {
        "clippy :)".to_string()
    }

    fn update(&mut self, message: Message) {

        let mut config = Ini::new();
        config.load("config.ini");
        match message {
            Message::Increase => {
                self.temp += 1;
            }
            Message::Decrease => {
                self.temp -= 1;
            }
        }
        config.set("Temp", "temp", Some(self.temp.to_string()));
        config.write("config.ini");
    }

    fn view(&self) -> Element<Message> {
        let mut list = column![];
        let button1 = Container::new(
            button(text("Increase").horizontal_alignment(alignment::Horizontal::Center))
                .on_press(Message::Increase),
        )
        .width(Length::Fill)
        .padding(5);
        let button2 = Container::new(
            button(text("Decrease").horizontal_alignment(alignment::Horizontal::Center))
                .on_press(Message::Decrease),
        )
        .width(Length::Fill)
        .padding(5);

        list = list.push(button1);
        list = list.push(button2);
        //column![button1, text(self.temp).size(50), button2]
        column![list, text(self.temp).size(50)]
            .padding(20)
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .width(Length::Fill)
            .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn style(&self) -> theme::Application {
        theme::Application::Default
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn should_exit(&self) -> bool {
        false
    }

    fn run(settings: Settings<()>) -> Result<(), iced::Error>
    where
        Self: 'static + Sized,
    {
        <Self as iced::Application>::run(settings)
    }
}

impl Default for Clipboard {
    fn default() -> Self {
        Self::new()
    }
}
