use crate::connect::ConnectNodeDto;
use crate::ok_client::RqClient;
use iced::{text_input, Command, Element, Row, Text, TextInput};

pub struct SendAmount {
    node: ConnectNodeDto,
    address: String,
    amount_input_state: text_input::State,
    amount_input_value: String,
    transaction: Option<String>,
    transaction_error: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Message {
    SetAmount(String),
    SendAmount(String),
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
            transaction: None,
            transaction_error: None,
        }
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::SetAmount(amount) => {
                self.amount_input_value = amount;
            }
            Message::SendAmount(amount) => {
                let send_amount_task = send_amount(self.address.clone(), amount, self.node.clone());

                return Command::perform(send_amount_task, |m| m);
            }
            Message::SetTransaction(transaction) => {
                self.transaction = Some(transaction);
            }
            Message::SetTransactionError(transaction_error) => {
                self.transaction_error = Some(transaction_error);
            }
        };

        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        Row::new()
            .padding(20)
            .push(Text::new(&format!("From: {} amount:", self.address)))
            .push(TextInput::new(
                &mut self.amount_input_state,
                "address to send",
                &self.amount_input_value,
                Message::SendAmount,
            ))
            .into()
    }
}

async fn send_amount(address: String, amount_string: String, node: ConnectNodeDto) -> Message {
    let amount_result = amount_string.parse::<f64>();

    if let Err(_) = amount_result {
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
    } else {
        Message::SetTransaction(result.result)
    }
}
