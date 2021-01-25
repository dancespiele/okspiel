use crate::connect::ConnectNodeModel;
use dirs::home_dir;
use sled::{Db, Error, IVec};
use std::str;
use tokio::fs::{create_dir, read_dir};

#[derive(Debug, Clone)]
pub struct ConnectionDB {
    db: Db,
}

impl ConnectionDB {
    pub async fn new() -> Self {
        let ok_spiel_dir = format!("{}/.okspiel", home_dir().unwrap().to_str().unwrap());
        if read_dir(&ok_spiel_dir).await.is_err() {
            create_dir(&ok_spiel_dir).await.unwrap();
        }

        let tree = sled::open(&ok_spiel_dir).unwrap();

        Self { db: tree }
    }

    pub fn get_connections(&self) -> Vec<ConnectNodeModel> {
        let connections_option = self.db.get("connections").unwrap();

        if let Some(connections) = connections_option {
            let connections_string = str::from_utf8(&connections).unwrap();
            serde_json::from_str(connections_string).unwrap()
        } else {
            vec![ConnectNodeModel::from((
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ))]
        }
    }

    pub fn insert_model(&self, key: String, model: String) -> Result<Option<IVec>, Error> {
        self.db.insert(key, model.as_bytes())
    }
}
