use crate::ok_client::Walletlocked;

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
    pub status: Walletlocked,
    pub staking: bool,
    pub connected: bool,
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

impl From<ConnectNodeDto> for ConnectNodeModel {
    fn from(connect_node_dto: ConnectNodeDto) -> Self {
        Self {
            name: connect_node_dto.name,
            address: connect_node_dto.address,
            account: connect_node_dto.account,
            username: connect_node_dto.username,
            password: connect_node_dto.password,
            phrase: connect_node_dto.phrase,
        }
    }
}

impl
    From<(
        String,
        String,
        String,
        String,
        String,
        String,
        Walletlocked,
        bool,
        bool,
    )> for ConnectNodeDto
{
    fn from(
        connect_node: (
            String,
            String,
            String,
            String,
            String,
            String,
            Walletlocked,
            bool,
            bool,
        ),
    ) -> Self {
        let (name, address, account, username, password, phrase, status, staking, connected) =
            connect_node;

        Self {
            name,
            address,
            account,
            username,
            password,
            phrase,
            status,
            staking,
            connected,
        }
    }
}
