use crate::connect::ConnectNodeDto;
use crate::ok_client::RqClient;
use iced::{button, text_input, Button, Checkbox, Column, Command, Element, Row, Text, TextInput};

#[derive(Debug, Clone)]
pub struct UnlockScreen {
    node: ConnectNodeDto,
    time_unlock_state: text_input::State,
    time_unlock_value: String,
    unlock_button_state: button::State,
    staking_only: bool,
    response: Option<String>,
    unlock_error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetTimeUnlock(String),
    SetStakingOnly(bool),
    Unlock(ConnectNodeDto),
    SetResponse(String),
    SetUnlockError(String),
}

impl UnlockScreen {
    pub fn new() -> Self {
        Self {
            node: ConnectNodeDto::from((
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                false,
                false,
            )),
            time_unlock_state: text_input::State::new(),
            time_unlock_value: "1000".to_string(),
            unlock_button_state: button::State::new(),
            staking_only: false,
            response: None,
            unlock_error: None,
        }
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::SetTimeUnlock(time) => {
                self.time_unlock_value = time;
            }
            Message::SetStakingOnly(value) => {
                self.staking_only = value;

                if self.staking_only {
                    self.time_unlock_value = "0".to_string();
                } else {
                    self.time_unlock_value = "1000".to_string();
                }
            }
            Message::Unlock(node) => {
                let unlock_wallet_task =
                    unlock_wallet(node, self.time_unlock_value.clone(), self.staking_only);

                return Command::perform(unlock_wallet_task, |m| m);
            }
            Message::SetResponse(response) => {
                self.response = Some(response);
            }
            Message::SetUnlockError(error) => {
                self.unlock_error = Some(error);
            }
        }

        Command::none()
    }

    pub fn set_node(&mut self, node: ConnectNodeDto) {
        self.node = node;
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .push::<Row<Message>>(
                Row::new()
                    .padding(20)
                    .push(Text::new("Time unlocked "))
                    .push::<Element<Message>>(if !self.staking_only {
                        TextInput::new(
                            &mut self.time_unlock_state,
                            "seconds",
                            &self.time_unlock_value,
                            Message::SetTimeUnlock,
                        )
                        .into()
                    } else {
                        Row::new().into()
                    })
                    .push(Checkbox::new(
                        self.staking_only,
                        "Staking only",
                        Message::SetStakingOnly,
                    ))
                    .spacing(10)
                    .push::<Button<Message>>(
                        Button::new(&mut self.unlock_button_state, Text::new("Unlock"))
                            .on_press(Message::Unlock(self.node.clone())),
                    ),
            )
            .push(Row::new().push(if self.response.is_some() {
                Text::new(self.response.clone().unwrap())
            } else {
                Text::new(self.unlock_error.clone().unwrap())
            }))
            .into()
    }
}

async fn unlock_wallet(node: ConnectNodeDto, time: String, staking_only: bool) -> Message {
    let rq_client = RqClient::new(
        node.address,
        node.account,
        node.username,
        node.password,
        node.phrase,
    );

    let response_result = rq_client
        .unlock_wallet(time.parse::<u32>().unwrap(), staking_only)
        .await;

    if let Ok(response) = response_result {
        if let Some(err_msg) = response.error {
            return Message::SetUnlockError(err_msg.message);
        }
        Message::SetResponse("locked successfully".to_string())
    } else {
        Message::SetUnlockError("Fail connecting with the rpc".to_string())
    }
}
