#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

mod connect;
mod db;
mod ok_client;
mod screens;

use crate::screens::{MainScreen, MainScreenMsg};
use iced::{executor, Application, Command, Element, Settings};

struct OkspielMainView {
    main_screen: MainScreen,
}

#[derive(Debug)]
pub enum Message {
    MainScreenMessage(MainScreenMsg),
}

impl Application for OkspielMainView {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (OkspielMainView, Command<Message>) {
        let main_screen = MainScreen::new();
        (OkspielMainView { main_screen }, Command::none())
    }

    fn title(&self) -> String {
        "Okspiel".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::MainScreenMessage(main_screen_msg) => {
                self.main_screen.update(main_screen_msg);
            }
        };

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        self.main_screen.view().map(Message::MainScreenMessage)
    }
}

fn main() -> iced::Result {
    OkspielMainView::run(Settings::default())
}
