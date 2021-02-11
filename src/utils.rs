use crate::connect::{ConnectNodeDto, ConnectNodeModel};
use crate::ok_client::RqClient;
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

        let wallet_info = rq_client.get_wallet_info().await.unwrap();

        let staking_infor = rq_client.get_staking_info().await.unwrap().result;

        ConnectNodeDto::from((
            c.name,
            c.address,
            c.account,
            c.username,
            c.password,
            c.phrase,
            wallet_info.result.unlocked_until.is_some()
                && wallet_info.result.unlocked_until.unwrap() > 0.0,
            staking_infor.enabled && staking_infor.staking,
        ))
    });

    connections_stream.collect::<Vec<ConnectNodeDto>>().await
}
