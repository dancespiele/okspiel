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

        let wallet_info = rq_client.get_wallet_info().await.unwrap().result;

        let staking_infor = rq_client.get_staking_info().await.unwrap().result;

        ConnectNodeDto::from((
            c.name,
            c.address,
            c.account,
            c.username,
            c.password,
            c.phrase,
            get_wallet_locked(&wallet_info.walletlocked) == Walletlocked::Locked,
            staking_infor.enabled && staking_infor.staking,
        ))
    });

    connections_stream.collect::<Vec<ConnectNodeDto>>().await
}

pub async fn get_one_connection_dto(connection_model: ConnectNodeModel) -> ConnectNodeDto {
    let rq_client = RqClient::new(
        connection_model.address.clone(),
        connection_model.account.clone(),
        connection_model.username.clone(),
        connection_model.password.clone(),
        connection_model.phrase.clone(),
    );

    let wallet_info = rq_client.get_wallet_info().await.unwrap().result;

    let staking_infor = rq_client.get_staking_info().await.unwrap().result;

    ConnectNodeDto::from((
        connection_model.name,
        connection_model.address,
        connection_model.account,
        connection_model.username,
        connection_model.password,
        connection_model.phrase,
        get_wallet_locked(&wallet_info.walletlocked) == Walletlocked::Locked,
        staking_infor.enabled && staking_infor.staking,
    ))
}

pub fn get_wallet_locked(value: &str) -> Walletlocked {
    match value {
        "Locked" => Walletlocked::Locked,
        "Unlocked" => Walletlocked::Unlocked,
        "Uncrypted" => Walletlocked::Uncrypted,
        &_ => Walletlocked::Uncrypted,
    }
}
