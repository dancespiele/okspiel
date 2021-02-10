#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod connect;
mod db;
mod node;
mod ok_client;
mod styles;
mod utils;

use crate::connect::{ConnectMsg, ConnectNode};
use crate::utils::get_connections_dto;
use db::ConnectionDB;
use iced::{executor, Application, Command, Element, Settings};

struct OkspielMainView {
    connect_node: ConnectNode,
}

#[derive(Debug)]
pub enum Message {
    ConnectMessage(ConnectMsg),
}

impl<'a> Application for OkspielMainView {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (OkspielMainView, Command<Message>) {
        let connections_task = get_connections();

        let connect_node = ConnectNode::new();
        (
            OkspielMainView { connect_node },
            Command::perform(connections_task, Message::ConnectMessage),
        )
    }

    fn title(&self) -> String {
        "Okspiel".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ConnectMessage(connect_node_msg) => self
                .connect_node
                .update(connect_node_msg)
                .map(Message::ConnectMessage),
        }
    }

    fn view(&mut self) -> Element<Message> {
        self.connect_node.view().map(Message::ConnectMessage)
    }
}

async fn get_connections() -> ConnectMsg {
    let connection_db = ConnectionDB::new().await;

    let connections = connection_db.get_connections();

    let connections_and_locked = get_connections_dto(connections).await;

    ConnectMsg::GetConnections(connections_and_locked)
}

fn main() -> iced::Result {
    OkspielMainView::run(Settings::default())
}
