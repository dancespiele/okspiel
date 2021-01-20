use crate::connect::{ConnectMsg, ConnectNode};
use iced::{button, Align, Button, Column, Container, Element, Length, Row, Text};

pub struct MainScreen {
    connect_node: ConnectNode,
    show_connect_config: bool,
    add_node: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    ShowConnectConfig,
    ConnectMessage(ConnectMsg),
}

impl MainScreen {
    pub fn new() -> Self {
        let connect_node = ConnectNode::new();

        Self {
            show_connect_config: false,
            add_node: button::State::new(),
            connect_node,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ShowConnectConfig => {
                self.show_connect_config = !self.show_connect_config;
            }
            Message::ConnectMessage(connect_msg) => {
                self.connect_node.update(connect_msg);
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Container::new(
            Row::new()
                .height(Length::Fill)
                .padding(20)
                .push(
                    Column::new()
                        .width(Length::FillPortion(1))
                        .align_items(Align::Center)
                        .push::<Element<Message>>(
                            Button::new(&mut self.add_node, Text::new("Add Node"))
                                .on_press(Message::ShowConnectConfig)
                                .into(),
                        ),
                )
                .push::<Column<Message>>(if self.show_connect_config {
                    Column::new()
                        .width(Length::FillPortion(3))
                        .align_items(Align::End)
                        .push::<Element<Message>>(
                            self.connect_node.view().map(Message::ConnectMessage),
                        )
                } else {
                    Column::new().width(Length::FillPortion(3))
                }),
        )
        .into()
    }
}
