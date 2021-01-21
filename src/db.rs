use crate::connect::ConnectNodeModel;
use dirs::home_dir;
use sled::{Db, Result};
use std::str;
use tokio::fs::{create_dir, read_dir};

pub async fn init_tree() -> Result<Db> {
    let ok_spiel_dir = format!("{}/.okspiel", home_dir().unwrap().to_str().unwrap());
    if read_dir(&ok_spiel_dir).await.is_err() {
        create_dir(&ok_spiel_dir).await.unwrap();
    }

    let tree = sled::open(&ok_spiel_dir)?;

    Ok(tree)
}

pub fn get_connections(db: Db) -> Vec<ConnectNodeModel> {
    let connections_option = db.get("connections").unwrap();

    if let Some(connections) = connections_option {
        let connections_string = str::from_utf8(&connections).unwrap();
        serde_json::from_str(connections_string).unwrap()
    } else {
        vec![ConnectNodeModel::from((
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
        ))]
    }
}
