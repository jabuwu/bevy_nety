use super::player::NetworkPlayer;
use bevy_nety_protocol::NetworkSocket;

pub struct NetworkClient {
    pub(crate) initialized: bool,
    pub(crate) socket: NetworkSocket,
    pub(crate) me: NetworkPlayer,
    pub(crate) players: Vec<NetworkPlayer>,
    pub(crate) existing_player_flag: bool,
}

impl NetworkClient {
    pub(crate) fn new(socket: NetworkSocket, me: NetworkPlayer) -> Self {
        Self {
            initialized: false,
            socket,
            me,
            players: vec![],
            existing_player_flag: true,
        }
    }
}
