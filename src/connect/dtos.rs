#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectNodeModel {
    pub name: String,
    pub address: String,
    pub account: String,
    pub username: String,
    pub password: String,
    pub phrase: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectNodeDto {
    pub name: String,
    pub address: String,
    pub account: String,
    pub username: String,
    pub password: String,
    pub phrase: String,
    pub locked: bool,
}

pub struct NodeLock {
    pub name: String,
    pub locked: bool,
}

impl From<(String, String, String, String, String, String)> for ConnectNodeModel {
    fn from(connect_node: (String, String, String, String, String, String)) -> Self {
        let (name, address, account, username, password, phrase) = connect_node;

        Self {
            name,
            address,
            account,
            username,
            password,
            phrase,
        }
    }
}

impl From<(String, String, String, String, String, String, bool)> for ConnectNodeDto {
    fn from(connect_node: (String, String, String, String, String, String, bool)) -> Self {
        let (name, address, account, username, password, phrase, locked) = connect_node;

        Self {
            name,
            address,
            account,
            username,
            password,
            phrase,
            locked,
        }
    }
}
