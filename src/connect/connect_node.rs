use iced::{button, text_input, Align, Button, Column, Element, Length, Row, Text, TextInput};

pub struct ConnectNode {
    address: text_input::State,
    address_value: String,
    username: text_input::State,
    username_value: String,
    password: text_input::State,
    password_value: String,
    phrase: text_input::State,
    phrase_value: String,
    connect: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetAddress(String),
    SetUsername(String),
    SetPassword(String),
    SetPhrase(String),
    Connect,
}

impl ConnectNode {
    pub fn new() -> Self {
        Self {
            address: text_input::State::new(),
            address_value: String::from(""),
            username: text_input::State::new(),
            username_value: String::from(""),
            password: text_input::State::new(),
            password_value: String::from(""),
            phrase: text_input::State::new(),
            phrase_value: String::from(""),
            connect: button::State::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SetAddress(addr) => {
                self.address_value = addr;
            }
            Message::SetUsername(username) => {
                self.username_value = username;
            }
            Message::SetPassword(pwd) => {
                self.password_value = pwd;
            }
            Message::SetPhrase(phase) => {
                self.phrase_value = phase;
            }
            Message::Connect => {
                self.address_value = String::from("");
                self.username_value = String::from("");
                self.password_value = String::from("");
                self.phrase_value = String::from("");
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .align_items(Align::Center)
            .padding(10)
            .push(Row::new().push(Text::new("Node Config Connection")))
            .push(
                Row::new()
                    .padding(20)
                    .spacing(10)
                    .push(Text::new("Address: "))
                    .push::<Element<Message>>(
                        TextInput::new(
                            &mut self.address,
                            "address node",
                            self.address_value.as_ref(),
                            Message::SetAddress,
                        )
                        .into(),
                    ),
            )
            .push(
                Row::new()
                    .padding(20)
                    .spacing(10)
                    .push(Text::new("RCP username: "))
                    .push::<Element<Message>>(
                        TextInput::new(
                            &mut self.username,
                            "rcp username node",
                            self.username_value.as_ref(),
                            Message::SetUsername,
                        )
                        .into(),
                    ),
            )
            .push(
                Row::new()
                    .padding(20)
                    .spacing(10)
                    .push(Text::new("RCP password: "))
                    .push::<Element<Message>>(
                        TextInput::new(
                            &mut self.password,
                            "rcp password node",
                            self.password_value.as_ref(),
                            Message::SetPassword,
                        )
                        .password()
                        .into(),
                    ),
            )
            .push(
                Row::new()
                    .padding(20)
                    .spacing(10)
                    .push(Text::new("Unlock phrase: "))
                    .push::<Element<Message>>(
                        TextInput::new(
                            &mut self.phrase,
                            "phrase",
                            self.phrase_value.as_ref(),
                            Message::SetPhrase,
                        )
                        .password()
                        .into(),
                    ),
            )
            .push(Row::new().padding(10).height(Length::FillPortion(2)).push(
                Button::new(&mut self.connect, Text::new("Connect")).on_press(Message::Connect),
            ))
            .into()
    }
}
