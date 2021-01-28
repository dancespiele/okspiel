use crate::connect::{ConnectMsg, ConnectNode, ConnectNodeModel};
use iced::{button, Button, Command, Container, Element, Row, Text};

pub struct NodeScreen {
    node_connection_data: ConnectNodeModel,
    connect_node: ConnectNode,
    delete_connection: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    Disconnect(ConnectMsg),
}

impl NodeScreen {
    pub fn new(connect_node_model: ConnectNodeModel) -> Self {
        let connect_node = ConnectNode::new();

        Self {
            node_connection_data: connect_node_model,
            delete_connection: button::State::new(),
            connect_node,
        }
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::Disconnect(connection_msg) => self
                .connect_node
                .update(connection_msg)
                .map(ConnectMsg::Disconnect),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Container::new(
            Row::new()
                .padding(10)
                .push(Text::new(self.node_connection_data.name.clone()))
                .push::<Element<Message>>(
                    Button::new(&mut self.delete_connection, Text::new('\u{1F5D1}'))
                        .on_press(Message::Disconnect(ConnectMsg::Disconnect(
                            self.node_connection_data.name.clone(),
                        )))
                        .into(),
                ),
        )
        .into()
    }
}
