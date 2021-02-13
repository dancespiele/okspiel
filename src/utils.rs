use crate::connect::{ConnectNodeDto, ConnectNodeModel};
use crate::ok_client::{RqClient, Walletlocked};
use futures::stream::{self, StreamExt};

pub async fn get_connections_dto(connections_model: Vec<ConnectNodeModel>) -> Vec<ConnectNodeDto> {
    let connections_stream = stream::iter(connections_model);
    let connections_stream = connections_stream.then(|c| async move {
        let rq_client = RqClient::new(
            c.address.clone(),
            c.account.clone(),
            c.username.clone(),
            c.password.clone(),
            c.phrase.clone(),
        );

        let wallet_info_result = rq_client.get_wallet_info().await;

        let staking_info_result = rq_client.get_staking_info().await;

        if let Ok(wallet_info) = wallet_info_result {
            if let Ok(staking_info) = staking_info_result {
                return ConnectNodeDto::from((
                    c.name,
                    c.address,
                    c.account,
                    c.username,
                    c.password,
                    c.phrase,
                    get_wallet_locked(&wallet_info.result.walletlocked) == Walletlocked::Locked,
                    staking_info.result.enabled && staking_info.result.staking,
                    true,
                ));
            }
        }

        ConnectNodeDto::from((
            c.name, c.address, c.account, c.username, c.password, c.phrase, false, false, false,
        ))
    });

    let connections_saved = connections_stream.collect::<Vec<ConnectNodeDto>>().await;

    connections_saved
        .into_iter()
        .filter(|c| c.connected)
        .collect::<Vec<ConnectNodeDto>>()
}

pub fn get_wallet_locked(value: &str) -> Walletlocked {
    match value {
        "Locked" => Walletlocked::Locked,
        "Unlocked" => Walletlocked::Unlocked,
        "Uncrypted" => Walletlocked::Uncrypted,
        &_ => Walletlocked::Uncrypted,
    }
}
