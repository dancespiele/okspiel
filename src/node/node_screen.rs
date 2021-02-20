use crate::connect::{ConnectMsg, ConnectNodeDto};
use crate::ok_client::Walletlocked;
use crate::styles::ButtonStyles;
use iced::{
    button, pick_list, Button, Column, Container, Element, Length, PickList, Row, Svg, Text,
};

#[derive(Debug, Clone)]
pub struct NodeScreen {
    pub node_connection_data: ConnectNodeDto,
    delete_connection: button::State,
    button_lock_state: button::State,
    pick_options: pick_list::State<NodeOptions>,
    selected_option: NodeOptions,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeOptions {
    NodeName(String),
    Info,
    Receive,
    Send,
}

impl NodeOptions {
    const OPTIONS: [NodeOptions; 3] = [NodeOptions::Info, NodeOptions::Receive, NodeOptions::Send];
}

impl std::fmt::Display for NodeOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NodeOptions::NodeName(name) => name,
                NodeOptions::Info => "Info",
                NodeOptions::Receive => "Receive",
                NodeOptions::Send => "Send",
            }
        )
    }
}

impl NodeScreen {
    pub fn new(connect_node_dto: ConnectNodeDto) -> Self {
        Self {
            pick_options: pick_list::State::default(),
            node_connection_data: connect_node_dto.clone(),
            delete_connection: button::State::new(),
            button_lock_state: button::State::new(),
            selected_option: NodeOptions::NodeName(connect_node_dto.name),
        }
    }

    pub fn set_selected_option(&mut self, selected_option: NodeOptions) {
        self.selected_option = selected_option;
    }

    pub fn view(&mut self) -> Element<ConnectMsg> {
        let node_name = self.node_connection_data.name.clone();

        Container::new(
            Row::new()
                .padding(10)
                .push::<Element<ConnectMsg>>(
                    PickList::new(
                        &mut self.pick_options,
                        &NodeOptions::OPTIONS[..],
                        Some(self.selected_option.clone()),
                        move |opt| ConnectMsg::SelectNodeOption(opt, node_name.clone()),
                    )
                    .into(),
                )
                .push::<Element<ConnectMsg>>(
                    Button::new(
                        &mut self.delete_connection,
                        Svg::from_path("assets/trash-2.svg"),
                    )
                    .style(ButtonStyles::Delete)
                    .height(Length::Units(30))
                    .on_press(ConnectMsg::Disconnect(
                        self.node_connection_data.name.clone(),
                    ))
                    .into(),
                )
                .spacing(10)
                .push::<Element<ConnectMsg>>(
                    if self.node_connection_data.status != Walletlocked::Uncrypted {
                        Button::new(
                            &mut self.button_lock_state,
                            if self.node_connection_data.status == Walletlocked::Locked {
                                Svg::from_path("assets/unlock.svg")
                            } else {
                                Svg::from_path("assets/lock.svg")
                            },
                        )
                        .style(
                            if self.node_connection_data.status == Walletlocked::Locked {
                                ButtonStyles::Warnning
                            } else {
                                ButtonStyles::Success
                            },
                        )
                        .height(Length::Units(30))
                        .on_press(
                            if self.node_connection_data.status == Walletlocked::Locked {
                                ConnectMsg::ShowUnlock(self.node_connection_data.clone())
                            } else {
                                ConnectMsg::Lock(self.node_connection_data.clone())
                            },
                        )
                        .into()
                    } else {
                        Column::new().into()
                    },
                ),
        )
        .into()
    }
}
