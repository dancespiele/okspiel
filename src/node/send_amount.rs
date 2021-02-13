use crate::connect::ConnectNodeDto;
use crate::ok_client::RqClient;
use iced::{button, text_input, Button, Column, Command, Element, Row, Text, TextInput};

use super::address;

pub struct SendAmount {
    node: ConnectNodeDto,
    address: String,
    amount_input_state: text_input::State,
    amount_input_value: String,
    to_address_state: text_input::State,
    to_address_value: String,
    send_amount_state: button::State,
    transaction: Option<String>,
    transaction_error: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Message {
    SetAmount(String),
    SetAddressToSend(String),
    SendAmount,
    SetTransaction(String),
    SetTransactionError(String),
}

impl SendAmount {
    pub fn new(address: String, node: ConnectNodeDto) -> Self {
        Self {
            node,
            address,
            amount_input_state: text_input::State::new(),
            amount_input_value: String::from(""),
            to_address_state: text_input::State::new(),
            to_address_value: String::from(""),
            send_amount_state: button::State::new(),
            transaction: None,
            transaction_error: None,
        }
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::SetAmount(amount) => {
                self.amount_input_value = amount;
            }
            Message::SetAddressToSend(address) => {
                self.to_address_value = address;
            }
            Message::SendAmount => {
                self.transaction = None;
                self.transaction_error = None;
                let send_amount_task = send_amount(
                    self.to_address_value.clone(),
                    self.amount_input_value.clone(),
                    self.node.clone(),
                );

                return Command::perform(send_amount_task, |m| m);
            }
            Message::SetTransaction(transaction) => {
                self.to_address_value = "".to_string();
                self.amount_input_value = "".to_string();
                self.transaction = Some(transaction);
            }
            Message::SetTransactionError(transaction_error) => {
                self.transaction_error = Some(transaction_error);
            }
        };

        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .push::<Row<Message>>(
                Row::new()
                    .padding(20)
                    .push(Text::new(&format!("From: {} amount:", self.address)))
                    .spacing(10)
                    .push(TextInput::new(
                        &mut self.amount_input_state,
                        "amount",
                        &self.amount_input_value,
                        Message::SetAmount,
                    ))
                    .spacing(10)
                    .push(Text::new("To:"))
                    .spacing(10)
                    .push(TextInput::new(
                        &mut self.to_address_state,
                        "Address to send",
                        &self.to_address_value,
                        Message::SetAddressToSend,
                    ))
                    .spacing(10)
                    .push(
                        Button::new(&mut self.send_amount_state, Text::new("Send"))
                            .on_press(Message::SendAmount),
                    ),
            )
            .padding(20)
            .push(Text::new(
                if let Some(transaction) = self.transaction.clone() {
                    format!("Transaction number: {}", transaction)
                } else if let Some(error) = self.transaction_error.clone() {
                    format!("Error to send transaction: {}", error)
                } else {
                    "".to_string()
                },
            ))
            .into()
    }
}

async fn send_amount(address: String, amount_string: String, node: ConnectNodeDto) -> Message {
    if amount_string.is_empty() {
        return Message::SetTransactionError("amount to send is required".to_string());
    } else if address.is_empty() {
        return Message::SetTransactionError("destination address is required".to_string());
    }

    let amount_result = amount_string.parse::<f64>();

    if amount_result.is_err() {
        return Message::SetTransactionError("Only number is supported".to_string());
    }

    let rq_client = RqClient::new(
        node.address.clone(),
        node.account.clone(),
        node.username.clone(),
        node.password.clone(),
        node.phrase.clone(),
    );

    let response = rq_client
        .send_to_address(address, amount_result.unwrap())
        .await;

    if let Err(error) = response {
        return Message::SetTransaction(error.to_string());
    }

    let result = response.unwrap();

    if let Some(error) = result.error {
        Message::SetTransactionError(error.message)
    } else if let Some(transaction) = result.result {
        Message::SetTransaction(transaction)
    } else {
        Message::SetTransactionError("Error to send Transaction".to_string())
    }
}
