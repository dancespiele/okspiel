use super::ConnectNodeModel;
use crate::db::ConnectionDB;
use crate::node::{NodeScreen, NodeScreenMsg};
use iced::{
    button, text_input, Align, Button, Column, Command, Container, Element, Length, Row, Text,
    TextInput,
};

pub struct ConnectNode {
    name: text_input::State,
    pub name_value: String,
    address: text_input::State,
    pub address_value: String,
    username: text_input::State,
    pub username_value: String,
    password: text_input::State,
    pub password_value: String,
    phrase: text_input::State,
    pub phrase_value: String,
    connect: button::State,
    connections_node_model: Vec<ConnectNodeModel>,
    show_connect_config: bool,
    add_node: button::State,
    node_screens: Vec<NodeScreen>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetName(String),
    SetAddress(String),
    SetUsername(String),
    SetPassword(String),
    SetPhrase(String),
    GetConnections(Vec<ConnectNodeModel>),
    Connect,
    ShowConnectConfig,
    DrawNodeScreen(usize, NodeScreenMsg),
}

impl ConnectNode {
    pub fn new() -> Self {
        Self {
            name: text_input::State::new(),
            name_value: String::from(""),
            address: text_input::State::new(),
            address_value: String::from(""),
            username: text_input::State::new(),
            username_value: String::from(""),
            password: text_input::State::new(),
            password_value: String::from(""),
            phrase: text_input::State::new(),
            phrase_value: String::from(""),
            connect: button::State::new(),
            connections_node_model: vec![],
            show_connect_config: false,
            add_node: button::State::new(),
            node_screens: vec![],
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ShowConnectConfig => {
                self.show_connect_config = !self.show_connect_config;
            }
            Message::SetName(name) => {
                self.name_value = name;
            }
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
            Message::GetConnections(connections) => {
                self.connections_node_model = connections.to_vec();

                for c in connections {
                    let node_screen = NodeScreen::new(c);
                    self.node_screens.push(node_screen);
                }
            }
            Message::Connect => {
                let add_connection_task = add_connection(
                    self.name_value.clone(),
                    self.address_value.clone(),
                    self.username_value.clone(),
                    self.password_value.clone(),
                    self.phrase_value.clone(),
                );

                return Command::perform(add_connection_task, Message::GetConnections);
            }
            Message::DrawNodeScreen(i, node_screen_msg) => {
                self.node_screens[i].update(node_screen_msg);
            }
        }

        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        Container::new(
            Row::new()
                .height(Length::Fill)
                .padding(20)
                .push::<Element<Message>>(
                    Column::new()
                        .width(Length::FillPortion(1))
                        .align_items(Align::Center)
                        .push::<Row<Message>>(self.node_screens.iter_mut().enumerate().fold(
                            Row::new().padding(5),
                            |row, (i, n)| {
                                row.push(n.view().map(move |m| Message::DrawNodeScreen(i, m)))
                            },
                        ))
                        .push::<Element<Message>>(
                            Button::new(&mut self.add_node, Text::new("Add Node"))
                                .on_press(Message::ShowConnectConfig)
                                .into(),
                        )
                        .into(),
                )
                .push::<Column<Message>>(if self.show_connect_config {
                    Column::new()
                        .width(Length::FillPortion(3))
                        .align_items(Align::End)
                        .push::<Column<Message>>(
                            Column::new()
                                .align_items(Align::Center)
                                .padding(10)
                                .push(Row::new().push(Text::new("Node Config Connection")))
                                .push(
                                    Row::new()
                                        .padding(20)
                                        .spacing(10)
                                        .push(Text::new("Wallet node name: "))
                                        .push::<Element<Message>>(
                                            TextInput::new(
                                                &mut self.name,
                                                "wallet node name",
                                                self.name_value.as_ref(),
                                                Message::SetName,
                                            )
                                            .into(),
                                        ),
                                )
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
                                .push(
                                    Row::new().padding(10).height(Length::FillPortion(2)).push(
                                        Button::new(&mut self.connect, Text::new("Connect"))
                                            .on_press(Message::Connect),
                                    ),
                                ),
                        )
                } else {
                    Column::new().width(Length::FillPortion(3))
                }),
        )
        .into()
    }
}

async fn add_connection(
    name: String,
    address: String,
    username: String,
    password: String,
    phrase_value: String,
) -> Vec<ConnectNodeModel> {
    let connection_db = ConnectionDB::new().await;

    let mut connections = connection_db.get_connections();

    connections.push(ConnectNodeModel::from((
        name,
        address,
        username,
        password,
        phrase_value,
    )));

    let connection_db_string = serde_json::to_string(&connections).unwrap();

    connection_db.insert_model("connections".to_string(), connection_db_string);

    connections
}
