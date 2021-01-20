use dirs::home_dir;
use sled::{Db, Result};
use tokio::fs::{create_dir, read_dir};

pub async fn init_tree() -> Result<Db> {
    let ok_spiel_dir = format!("{}/.okspiel", home_dir().unwrap().to_str().unwrap());
    if read_dir(&ok_spiel_dir).await.is_err() {
        create_dir(&ok_spiel_dir).await.unwrap();
    }

    let tree = sled::open(&ok_spiel_dir)?;

    Ok(tree)
}
