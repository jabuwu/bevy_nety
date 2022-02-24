use crate::{
    events::NetworkEventTraits,
    messages::NetworkMessage,
    player::NetworkPlayer,
    serialized_struct::{NetworkSerializedStruct, NetworkSerializedStructMap},
};
use bevy_nety_protocol::NetworkSocket;

pub(crate) struct NetworkClientPlayer {
    pub(crate) handle: NetworkPlayer,
    pub(crate) data: NetworkSerializedStructMap,
}

pub struct NetworkClient {
    pub(crate) initialized: bool,
    pub(crate) socket: NetworkSocket,
    pub(crate) me: NetworkPlayer,
    pub(crate) players: Vec<NetworkClientPlayer>,
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

    pub fn send<T>(&mut self, event: T)
    where
        T: NetworkEventTraits,
    {
        self.socket.send(
            NetworkMessage::Event {
                data: NetworkSerializedStruct::from_struct(&event),
            }
            .serialize(),
        );
    }

    pub(crate) fn players(&self) -> Vec<NetworkPlayer> {
        self.players.iter().map(|p| p.handle).collect()
    }
}
