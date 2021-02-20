use crate::styles::ButtonStyles;
use copypasta::{ClipboardContext, ClipboardProvider};
use iced::{button, Button, Element, HorizontalAlignment, Length, Row, Text};

pub struct Address {
    address: String,
    clipboard: ClipboardContext,
    button: button::State,
}

#[derive(Clone, Debug)]
pub enum Message {
    CopyAddress(String),
}

impl Address {
    pub fn new(address: String) -> Self {
        Self {
            address,
            clipboard: ClipboardContext::new().unwrap(),
            button: button::State::new(),
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::CopyAddress(address) => self.clipboard.set_contents(address).unwrap(),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Row::new()
            .padding(20)
            .push(Text::new(self.address.clone()).width(Length::FillPortion(3)))
            .push(
                Button::new(
                    &mut self.button,
                    Text::new("Copy").horizontal_alignment(HorizontalAlignment::Center),
                )
                .on_press(Message::CopyAddress(self.address.clone()))
                .style(ButtonStyles::Success)
                .padding(3)
                .width(Length::FillPortion(2)),
            )
            .into()
    }
}
