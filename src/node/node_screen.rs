use crate::connect::{ConnectMsg, ConnectNodeModel};
use crate::styles::ButtonStyles;
use iced::{button, pick_list, Button, Container, Element, PickList, Row, Svg, Text};

#[derive(Debug, Clone)]
pub struct NodeScreen {
    pub node_connection_data: ConnectNodeModel,
    delete_connection: button::State,
    button_lock_state: button::State,
    pick_options: pick_list::State<NodeOptions>,
    selected_option: NodeOptions,
    is_locked: bool,
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
    pub fn new(connect_node_model: ConnectNodeModel) -> Self {
        Self {
            pick_options: pick_list::State::default(),
            node_connection_data: connect_node_model.clone(),
            delete_connection: button::State::new(),
            button_lock_state: button::State::new(),
            selected_option: NodeOptions::NodeName(connect_node_model.name),
            is_locked: true,
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
                    Button::new(&mut self.delete_connection, Text::new("DELETE"))
                        .style(ButtonStyles::Delete)
                        .on_press(ConnectMsg::Disconnect(
                            self.node_connection_data.name.clone(),
                        ))
                        .into(),
                )
                .push::<Element<ConnectMsg>>(
                    Button::new(
                        &mut self.button_lock_state,
                        Svg::from_path(if self.is_locked {
                            "/assets/lock.svg"
                        } else {
                            "/assets/inlo.svg"
                        })
                        .into(),
                    )
                    .on_press(if self.is_locked {
                        ConnectMsg::Unlock
                    } else {
                        ConnectMsg::Lock
                    })
                    .into(),
                ),
        )
        .into()
    }
}
