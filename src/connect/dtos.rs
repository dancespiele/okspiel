use super::{connect_node, ConnectNode};
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectNodeModel {
    address: String,
    username: String,
    password: String,
    phrase: String,
}

impl From<(String, String, String, String)> for ConnectNodeModel {
    fn from(connect_node: (String, String, String, String)) -> Self {
        let (address, username, password, phrase) = connect_node;

        Self {
            address,
            username,
            password,
            phrase,
        }
    }
}
