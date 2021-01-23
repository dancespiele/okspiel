use crate::connect::ConnectNodeModel;
use iced::{Container, Element, Row, Text};

pub struct NodeScreen {
    node_connection_data: ConnectNodeModel,
}

#[derive(Debug, Clone)]
pub enum Message {
    GetWalletInfo,
}

impl NodeScreen {
    pub fn new(connect_node_model: ConnectNodeModel) -> Self {
        Self {
            node_connection_data: connect_node_model,
        }
    }

    pub fn update(&self, msg: Message) {
        match msg {
            Message::GetWalletInfo => {}
        }
    }

    pub fn view(&self) -> Element<Message> {
        Container::new(
            Row::new()
                .padding(10)
                .push(Text::new(self.node_connection_data.name.clone())),
        )
        .into()
    }
}
