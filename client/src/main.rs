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
use std::process::exit;
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
    Cp(String),
}
struct Clipboard {
    cp_status: bool,
    cp_data: String,
    history: String,
}
impl Sandbox for Clipboard {
    type Message = Message;

    fn new() -> Self {
        if !Path::new("clip_history").exists() {
            println!("Please start daemon first");
            exit(404);
        }
        let mut config = Ini::new();
        config.load("clip_history");
        Clipboard {
            cp_data: config.get("Clipboard", "cpData").unwrap(),
            cp_status: config
                .getboolcoerce("Clipboard", "cpStatus")
                .unwrap()
                .unwrap(),
            history: config.get("Clipboard", "history").unwrap(),
        }
    }

    fn title(&self) -> String {
        "clippy :)".to_string()
    }

    fn update(&mut self, message: Message) {
        let mut config = Ini::new();
        config.load("clip_history");
        match message {
            Message::Cp(s) => {
                let mut config = Ini::new();
                config.load("clip_history");
                config.set("Clipboard", "cpData", Some(s));
                config.set("Clipboard", "cpStatus", Some("1".to_string()));
                config.write("clip_history");
                exit(0);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let mut config = Ini::new();
        config.load("clip_history");
        let mut list = column![];
        let data = config.get("Clipboard", "history").unwrap();
        let mut list_data: Vec<String> = data
            .split(" (MADE_BY_CELIBISTRIAL) ")
            .map(|s| s.to_string())
            .collect();
        list_data.reverse();
        println!("{:?}", list_data);
        for i in list_data {
            list = list.push(button(text(i.clone())).on_press(Message::Cp(i)));
        }
        //        let button1 = Container::new(
        //            button(text("Increase").horizontal_alignment(alignment::Horizontal::Center))
        //                .on_press(Message::Increase),
        //        )
        //        .width(Length::Fill)
        //        .padding(5);
        //       column![list, text(self.temp).size(50)]
        //            .padding(20)
        //            .spacing(20)
        //            .align_items(iced::Alignment::Center)
        //            .width(Length::Fill)
        //            .into()
        list.padding(20)
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
