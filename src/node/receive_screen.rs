use super::{Address, AddressMessage};
use iced::{button, scrollable, Column, Element, Row, Scrollable};

pub struct ReceiveScreen {
    pub addresses: Vec<Address>,
    pub buttons: Vec<button::State>,
    pub scroll: scrollable::State,
}

#[derive(Clone, Debug)]
pub enum Message {
    AddressMsg(usize, AddressMessage),
}

impl ReceiveScreen {
    pub fn new() -> Self {
        Self {
            addresses: vec![],
            buttons: vec![],
            scroll: scrollable::State::new(),
        }
    }

    pub fn set_address(&mut self, addresses: Vec<String>) {
        if !self.addresses.is_empty() {
            self.addresses = vec![];
        }
        for address in addresses.into_iter() {
            let address = Address::new(address);
            self.addresses.push(address);
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::AddressMsg(index, address_message) => {
                self.addresses[index].update(address_message)
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .push(
                Scrollable::new(&mut self.scroll).push::<Element<Message>>(
                    self.addresses
                        .iter_mut()
                        .enumerate()
                        .fold(Row::new().padding(20), move |r, address| {
                            let (i, a) = address;
                            r.push(a.view().map(move |m| Message::AddressMsg(i, m)))
                        })
                        .into(),
                ),
            )
            .into()
    }
}
