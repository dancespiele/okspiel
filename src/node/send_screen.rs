use super::{SendAmount, SendAmountMsg};
use crate::connect::ConnectNodeDto;
use crate::ok_client::Walletlocked;
use iced::{Column, Command, Element, Row, Text};

pub struct SendScreen {
    senders: Vec<SendAmount>,
    status: Walletlocked,
}

#[derive(Clone, Debug)]
pub enum Message {
    SendAmountMessage(usize, SendAmountMsg),
}

impl SendScreen {
    pub fn new() -> Self {
        Self {
            senders: vec![],
            status: Walletlocked::Locked,
        }
    }

    pub fn set_addresses(&mut self, addresses: Vec<String>, node: ConnectNodeDto) {
        if !self.senders.is_empty() {
            self.senders = vec![];
        }

        self.status = node.status.clone();

        addresses.into_iter().for_each(|address| {
            let sender = SendAmount::new(address, node.clone());

            self.senders.push(sender);
        });
    }

    pub fn set_locked(&mut self, node: ConnectNodeDto) {
        self.status = node.status;
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::SendAmountMessage(index, send_amount_msg) => self.senders[index]
                .update(send_amount_msg)
                .map(move |m| Message::SendAmountMessage(index, m)),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        if self.status == Walletlocked::Locked {
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
