use super::{ConnectNodeDto, ConnectNodeModel};
use crate::db::ConnectionDB;
use crate::node::{
    NodeOptions, NodeScreen, ReceiveMessage, ReceiveScreen, SendScreen, SendScreenMsg,
};
use crate::ok_client::{RqClient, WalletInfo, Walletlocked};
use crate::styles::ButtonStyles;
use crate::utils::get_connections_dto;
use iced::{
    button, scrollable, text_input, Align, Button, Checkbox, Column, Command, Container, Element,
    Length, Row, Text, TextInput,
};

pub struct ConnectNode {
    name: text_input::State,
    pub name_value: String,
    url: text_input::State,
    pub url_value: String,
    account: text_input::State,
    pub account_value: String,
    username: text_input::State,
    pub username_value: String,
    password: text_input::State,
    pub password_value: String,
    phrase: text_input::State,
    pub phrase_value: String,
    connect: button::State,
    connections_node_model: Vec<ConnectNodeDto>,
    show_connect_config: bool,
    add_node: button::State,
    node_screens: Vec<NodeScreen>,
    show_connecion_error: (bool, String),
    show_disconnect_error: (bool, String),
    node_info: Option<WalletInfo>,
    receive_screen: ReceiveScreen,
    send_screen: SendScreen,
    show_option: Option<NodeOptions>,
    scroll: scrollable::State,
    show_unlock: bool,
    time_unlock_state: text_input::State,
    time_unlock_value: String,
    unlock_button_state: button::State,
    staking_only: bool,
    current_node: ConnectNodeDto,
    loading: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetName(String),
    SetUrl(String),
    SetAccount(String),
    SetUsername(String),
    SetPassword(String),
    SetPhrase(String),
    GetConnections(Vec<ConnectNodeDto>),
    Connect,
    ShowConnectConfig,
    SetConnectionError(String),
    Disconnect(String),
    SelectNodeOption(NodeOptions, String),
    ShowInfo(WalletInfo),
    ShowAddresses(Vec<String>, ConnectNodeDto),
    ReceiveMsg(ReceiveMessage),
    SendScreenMessage(SendScreenMsg),
    Lock(ConnectNodeDto),
    ShowUnlock(ConnectNodeDto),
    SetTimeUnlock(String),
    SetStakingOnly(bool),
    Unlock(ConnectNodeDto),
}

impl ConnectNode {
    pub fn new() -> Self {
        Self {
            name: text_input::State::new(),
            name_value: String::from(""),
            url: text_input::State::new(),
            url_value: String::from(""),
            account: text_input::State::new(),
            account_value: String::from(""),
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
            show_connecion_error: (false, String::from("")),
            show_disconnect_error: (false, String::from("")),
            node_info: None,
            show_option: None,
            receive_screen: ReceiveScreen::new(),
            send_screen: SendScreen::new(),
            scroll: scrollable::State::new(),
            show_unlock: false,
            time_unlock_state: text_input::State::new(),
            time_unlock_value: "1000".to_string(),
            unlock_button_state: button::State::new(),
            staking_only: false,
            current_node: ConnectNodeDto::from((
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                Walletlocked::Unlocked,
                false,
                false,
            )),
            loading: false,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ShowConnectConfig => {
                if !self.show_connect_config {
                    self.show_unlock = false;
                    self.remove_selected();
                }

                self.show_connect_config = !self.show_connect_config;
            }
            Message::SetName(name) => {
                self.name_value = name;
            }
            Message::SetUrl(addr) => {
                self.url_value = addr;
            }
            Message::SetAccount(account) => {
                self.account_value = account;
            }
            Message::SetUsername(username) => {
                self.username_value = username;
            }
            Message::SetPassword(pwd) => {
                self.password_value = pwd;
            }
            Message::ShowInfo(info) => {
                self.loading = false;
                self.node_info = Some(info);
            }
            Message::ShowAddresses(addresses, node) => {
                self.loading = false;
                if let Some(option) = self.show_option.clone() {
                    match option {
                        NodeOptions::Receive => {
                            self.receive_screen.set_address(addresses);
                        }
                        NodeOptions::Send => self.send_screen.set_addresses(addresses, node),
                        _ => (),
                    }
                }
            }
            Message::SelectNodeOption(node_selected, name) => {
                let position_option = self.get_position(name);
                self.remove_messages();
                self.remove_selected();
                self.show_unlock = false;
                self.loading = true;

                if let Some(position) = position_option {
                    match node_selected {
                        NodeOptions::Info => {
                            self.show_option = Some(NodeOptions::Info);
                            self.node_screens[position].set_selected_option(node_selected);
                            let node_info_task =
                                get_info(self.node_screens[position].node_connection_data.clone());

                            return Command::perform(node_info_task, |m| m);
                        }
                        NodeOptions::Receive => {
                            self.show_option = Some(NodeOptions::Receive);
                            self.node_screens[position].set_selected_option(node_selected);
                            let receive_task = list_addresses(
                                self.node_screens[position].node_connection_data.clone(),
                            );

                            return Command::perform(receive_task, |m| m);
                        }
                        NodeOptions::Send => {
                            self.show_option = Some(NodeOptions::Send);
                            self.node_screens[position].set_selected_option(node_selected);

                            let receive_task = list_addresses(
                                self.node_screens[position].node_connection_data.clone(),
                            );

                            return Command::perform(receive_task, |m| m);
                        }
                        _ => (),
                    }
                }
            }
            Message::SetPhrase(phase) => {
                self.phrase_value = phase;
            }
            Message::SetConnectionError(error) => {
                self.loading = false;
                self.show_connecion_error = (true, error);
            }
            Message::GetConnections(ref connections) => {
                self.connections_node_model = connections.to_vec();
                self.name_value = String::from("");
                self.url_value = String::from("");
                self.account_value = String::from("");
                self.username_value = String::from("");
                self.password_value = String::from("");
                self.phrase_value = String::from("");
                self.show_unlock = false;
                let mut node_screens: Vec<NodeScreen> = vec![];
                self.loading = false;

                self.send_screen.set_locked(self.current_node.clone());

                for c in connections {
                    let node_screen = NodeScreen::new(c.clone());
                    node_screens.push(node_screen);
                }

                self.node_screens = node_screens;
            }
            Message::Connect => {
                self.show_connecion_error = (false, String::from(""));

                let add_connection_task = add_connection(
                    self.name_value.clone(),
                    self.url_value.clone(),
                    self.account_value.clone(),
                    self.username_value.clone(),
                    self.password_value.clone(),
                    self.phrase_value.clone(),
                );

                self.loading = true;
                return Command::perform(add_connection_task, |m| m);
            }
            Message::Disconnect(name) => {
                self.show_disconnect_error = (false, String::from(""));
                self.loading = true;
                let delete_connection_task = delete_connection(name);

                return Command::perform(delete_connection_task, |m| m);
            }
            Message::ReceiveMsg(receive_message) => self.receive_screen.update(receive_message),
            Message::SendScreenMessage(send_message) => {
                return self
                    .send_screen
                    .update(send_message)
                    .map(Message::SendScreenMessage);
            }
            Message::ShowUnlock(node) => {
                self.show_unlock = true;
                self.show_connect_config = false;
                self.show_option = None;
                self.current_node = node;
            }
            Message::Lock(node) => {
                let lock_wallet_task = lock_wallet(node);
                self.loading = true;

                return Command::perform(lock_wallet_task, |m| m);
            }
            Message::SetTimeUnlock(time) => {
                self.time_unlock_value = time;
            }
            Message::SetStakingOnly(value) => {
                self.staking_only = value;

                if self.staking_only {
                    self.time_unlock_value = "0".to_string();
                } else {
                    self.time_unlock_value = "1000".to_string();
                }
            }
            Message::Unlock(node) => {
                let unlock_wallet_task =
                    unlock_wallet(node, self.time_unlock_value.clone(), self.staking_only);

                self.loading = true;
                return Command::perform(unlock_wallet_task, |m| m);
            }
        }

        Command::none()
    }

    fn remove_messages(&mut self) {
        self.show_connecion_error = (false, "".to_string());
        self.show_disconnect_error = (false, "".to_string());
        self.show_connect_config = false;
    }

    fn remove_selected(&mut self) {
        self.show_option = None;
        for (i, _) in self.node_screens.clone().into_iter().enumerate() {
            let node_name = self.node_screens[i].node_connection_data.name.clone();
            self.node_screens[i].set_selected_option(NodeOptions::NodeName(node_name))
        }
    }

    fn get_position(&self, name: String) -> Option<usize> {
        self.node_screens
            .clone()
            .into_iter()
            .position(|ns| *ns.node_connection_data.name == name)
    }

    pub fn view(&mut self) -> Element<Message> {
        Container::new(
            Row::new()
                .height(Length::Fill)
                .padding(20)
                .push::<Element<Message>>(
                    Column::new()
                        .width(Length::Units(400))
                        .align_items(Align::Center)
                        .push::<Column<Message>>(
                            self.node_screens
                                .iter_mut()
                                .fold(Column::new().padding(5), |column, n| column.push(n.view())),
                        )
                        .push::<Element<Message>>(
                            Button::new(&mut self.add_node, Text::new("Add Node"))
                                .style(ButtonStyles::Confirm)
                                .on_press(Message::ShowConnectConfig)
                                .into(),
                        )
                        .into(),
                )
                .push::<Column<Message>>(if self.loading {
                    Column::new()
                        .width(Length::Fill)
                        .padding(50)
                        .align_items(Align::Center)
                        .push(Text::new("LOADING"))
                } else if self.show_connect_config {
                    Column::new()
                        .width(Length::Fill)
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
                                                &mut self.url,
                                                "url node",
                                                self.url_value.as_ref(),
                                                Message::SetUrl,
                                            )
                                            .into(),
                                        ),
                                )
                                .push(
                                    Row::new()
                                        .padding(20)
                                        .spacing(10)
                                        .push(Text::new("Account: "))
                                        .push::<Element<Message>>(
                                            TextInput::new(
                                                &mut self.account,
                                                "account",
                                                self.account_value.as_ref(),
                                                Message::SetAccount,
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
                                )
                                .push(if self.show_connecion_error.0 {
                                    Row::new()
                                        .padding(10)
                                        .height(Length::FillPortion(2))
                                        .push(Text::new(self.show_connecion_error.1.clone()))
                                } else {
                                    Row::new()
                                }),
                        )
                } else if let Some(option) = self.show_option.clone() {
                    match option {
                        NodeOptions::Info => {
                            if let Some(node_info) = self.node_info.clone() {
                                Column::new()
                                    .padding(20)
                                    .push(
                                        Row::new()
                                            .padding(20)
                                            .spacing(10)
                                            .push(
                                                Column::new()
                                                    .width(Length::FillPortion(2))
                                                    .push(Text::new("Node version: ")),
                                            )
                                            .push(
                                                Column::new().width(Length::FillPortion(2)).push(
                                                    Text::new(&format!(
                                                        "{} v",
                                                        node_info.walletversion
                                                    )),
                                                ),
                                            ),
                                    )
                                    .push(
                                        Row::new()
                                            .padding(20)
                                            .spacing(10)
                                            .push(
                                                Column::new()
                                                    .width(Length::FillPortion(2))
                                                    .push(Text::new("Balance: ")),
                                            )
                                            .push(
                                                Column::new().width(Length::FillPortion(2)).push(
                                                    Text::new(&format!(
                                                        "{} $OK",
                                                        node_info.balance
                                                    )),
                                                ),
                                            ),
                                    )
                                    .push(
                                        Row::new()
                                            .padding(20)
                                            .spacing(10)
                                            .push(
                                                Column::new()
                                                    .width(Length::FillPortion(2))
                                                    .push(Text::new("Status: ")),
                                            )
                                            .push(
                                                Column::new()
                                                    .width(Length::FillPortion(2))
                                                    .push(Text::new(node_info.walletlocked)),
                                            ),
                                    )
                            } else {
                                Column::new().width(Length::FillPortion(3))
                            }
                        }
                        NodeOptions::Receive => Column::new().padding(20).push::<Element<Message>>(
                            self.receive_screen.view().map(Message::ReceiveMsg),
                        ),
                        NodeOptions::Send => Column::new().padding(20).push::<Element<Message>>(
                            self.send_screen.view().map(Message::SendScreenMessage),
                        ),
                        _ => Column::new().width(Length::FillPortion(3)),
                    }
                } else if self.show_unlock {
                    Column::new()
                        .padding(20)
                        .width(Length::FillPortion(3))
                        .push::<Row<Message>>(
                            Row::new()
                                .padding(20)
                                .push(Text::new("Time unlocked "))
                                .push::<Element<Message>>(if !self.staking_only {
                                    TextInput::new(
                                        &mut self.time_unlock_state,
                                        "seconds",
                                        &self.time_unlock_value,
                                        Message::SetTimeUnlock,
                                    )
                                    .into()
                                } else {
                                    Row::new().into()
                                })
                                .push(Checkbox::new(
                                    self.staking_only,
                                    "Staking only",
                                    Message::SetStakingOnly,
                                ))
                                .spacing(10)
                                .push::<Button<Message>>(
                                    Button::new(&mut self.unlock_button_state, Text::new("Unlock"))
                                        .on_press(Message::Unlock(self.current_node.clone())),
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
    url: String,
    account: String,
    username: String,
    password: String,
    phrase: String,
) -> Message {
    if name.is_empty() {
        return Message::SetConnectionError("name of connection is required".to_string());
    } else if url.is_empty() {
        return Message::SetConnectionError("node url is required".to_string());
    } else if account.is_empty() {
        return Message::SetConnectionError("account is required".to_string());
    } else if username.is_empty() {
        return Message::SetConnectionError("username is required".to_string());
    } else if password.is_empty() {
        return Message::SetConnectionError("password is required".to_string());
    }

    let rq_client = RqClient::new(
        url.clone(),
        account.clone(),
        username.clone(),
        password.clone(),
        phrase.clone(),
    );

    let response_connection = rq_client.get_wallet_info().await;

    if let Err(connection_error) = response_connection {
        return Message::SetConnectionError(connection_error.to_string());
    }

    if let Some(err_msg) = response_connection.unwrap().error {
        return Message::SetConnectionError(err_msg.message);
    }

    let connection_db = ConnectionDB::new().await;

    let mut connections = connection_db.get_connections();

    connections.push(ConnectNodeModel::from((
        name, url, account, username, password, phrase,
    )));

    let connection_db_string_result = serde_json::to_string(&connections);

    if let Err(serde_error) = connection_db_string_result {
        return Message::SetConnectionError(serde_error.to_string());
    }

    let response_result = connection_db.insert_model(
        "connections".to_string(),
        connection_db_string_result.unwrap(),
    );

    if let Err(response) = response_result {
        Message::SetConnectionError(response.to_string())
    } else {
        let connections_and_locked = get_connections_dto(connections).await;

        Message::GetConnections(connections_and_locked)
    }
}

async fn get_info(node: ConnectNodeDto) -> Message {
    let rq_client = RqClient::new(
        node.address.clone(),
        node.account.clone(),
        node.username.clone(),
        node.password.clone(),
        node.phrase.clone(),
    );

    let info_result = rq_client.get_wallet_info().await;

    if let Ok(info) = info_result {
        if let Some(err_msg) = info.error {
            return Message::SetConnectionError(err_msg.message);
        }
        Message::ShowInfo(info.result)
    } else {
        Message::SetConnectionError("Error to get node info".to_string())
    }
}

async fn delete_connection(name: String) -> Message {
    let connection_db = ConnectionDB::new().await;

    let connections = connection_db.get_connections();

    let connections_filtered = connections
        .into_iter()
        .filter(|c| *c.name != name)
        .collect::<Vec<ConnectNodeModel>>();

    let connection_db_string_result = serde_json::to_string(&connections_filtered);

    if let Err(serde_error) = connection_db_string_result {
        return Message::SetConnectionError(serde_error.to_string());
    }

    let response_result = connection_db.insert_model(
        "connections".to_string(),
        connection_db_string_result.unwrap(),
    );

    if let Err(response) = response_result {
        Message::SetConnectionError(response.to_string())
    } else {
        let connections_and_locked = get_connections_dto(connections_filtered).await;

        Message::GetConnections(connections_and_locked)
    }
}

async fn list_addresses(node: ConnectNodeDto) -> Message {
    let rq_client = RqClient::new(
        node.address.clone(),
        node.account.clone(),
        node.username.clone(),
        node.password.clone(),
        node.phrase.clone(),
    );

    let addresses_result = rq_client.get_addresses().await;

    if let Ok(addresses) = addresses_result {
        if let Some(err_msg) = addresses.error {
            return Message::SetConnectionError(err_msg.message);
        }
        Message::ShowAddresses(addresses.result, node)
    } else {
        Message::SetConnectionError("Error to get node info".to_string())
    }
}

async fn lock_wallet(node: ConnectNodeDto) -> Message {
    let rq_client = RqClient::new(
        node.address.clone(),
        node.account.clone(),
        node.username.clone(),
        node.password.clone(),
        node.phrase.clone(),
    );

    let locked_result = rq_client.lock_wallet().await;

    if let Err(locked_error) = locked_result {
        return Message::SetConnectionError(locked_error.to_string());
    }

    if let Some(locked_error) = locked_result.unwrap().error {
        Message::SetConnectionError(locked_error.message)
    } else {
        let connection_db = ConnectionDB::new().await;

        let connections = connection_db.get_connections();

        let connections_and_locked = get_connections_dto(connections).await;

        Message::GetConnections(connections_and_locked)
    }
}

async fn unlock_wallet(node: ConnectNodeDto, time: String, staking_only: bool) -> Message {
    let rq_client = RqClient::new(
        node.address.clone(),
        node.account.clone(),
        node.username.clone(),
        node.password.clone(),
        node.phrase.clone(),
    );

    let response_result = rq_client
        .unlock_wallet(time.parse::<u32>().unwrap(), staking_only)
        .await;

    if let Ok(response) = response_result {
        if let Some(err_msg) = response.error {
            return Message::SetConnectionError(err_msg.message);
        }
        let connection_db = ConnectionDB::new().await;

        let connections = connection_db.get_connections();

        let connections_and_locked = get_connections_dto(connections).await;
        Message::GetConnections(connections_and_locked)
    } else {
        Message::SetConnectionError("Fail connecting with the rpc".to_string())
    }
}
