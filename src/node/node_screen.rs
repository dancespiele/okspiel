use crate::connect::{ConnectMsg, ConnectNodeModel};
use iced::{button, Button, Container, Element, Row, Text};

pub struct NodeScreen {
    node_connection_data: ConnectNodeModel,
    delete_connection: button::State,
}

impl NodeScreen {
    pub fn new(connect_node_model: ConnectNodeModel) -> Self {
        Self {
            node_connection_data: connect_node_model,
            delete_connection: button::State::new(),
        }
    }

    pub fn view(&mut self) -> Element<ConnectMsg> {
        Container::new(
            Row::new()
                .padding(10)
                .push(Text::new(self.node_connection_data.name.clone()))
                .push::<Element<ConnectMsg>>(
                    Button::new(&mut self.delete_connection, Text::new('\u{1F5D1}'))
                        .on_press(ConnectMsg::Disconnect(
                            self.node_connection_data.name.clone(),
                        ))
                        .into(),
                ),
        )
        .into()
    }
}
