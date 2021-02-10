use super::{send_amount, SendAmount, SendAmountMsg};
use crate::connect::ConnectNodeModel;
use iced::{Column, Command, Element, Row, Text};

pub struct SendScreen {
    addresses: Vec<String>,
    senders: Vec<SendAmount>,
    is_locked: bool,
}

#[derive(Clone)]
enum Message {
    SendAmountMessage(usize, SendAmountMsg),
}

impl SendScreen {
    pub fn new() -> Self {
        Self {
            addresses: vec![],
            senders: vec![],
            is_locked: false,
        }
    }

    pub fn set_lock(&mut self, is_locked: bool) {
        self.is_locked = is_locked;
    }

    pub fn set_addresses(&mut self, addresses: Vec<String>, node_model: ConnectNodeModel) {
        if !self.senders.is_empty() {
            self.senders = vec![];
        }
        addresses.into_iter().for_each(|address| {
            let sender = SendAmount::new(address, node_model.clone());

            self.senders.push(sender);
        });
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::SendAmountMessage(index, send_amount_msg) => self.senders[index]
                .update(send_amount_msg)
                .map(move |m| Message::SendAmountMessage(index, m)),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        if self.is_locked {
            Row::new()
                .padding(20)
                .push(Text::new(
                    "You need to unlock the wallet in order to send amount",
                ))
                .into()
        } else {
            self.senders
                .iter_mut()
                .enumerate()
                .fold(Column::new(), |r, a| {
                    let (index, sender) = a;

                    r.push::<Element<Message>>(
                        sender
                            .view()
                            .map(move |m| Message::SendAmountMessage(index, m)),
                    )
                })
                .into()
        }
    }
}
