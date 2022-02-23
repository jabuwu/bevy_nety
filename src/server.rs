use super::messages::NetworkMessage;
use super::player::NetworkPlayer;
use bevy_nety_protocol::{NetworkHost, NetworkSocket};

pub(crate) struct NetworkServerJoiner {
    pub(crate) socket: Option<NetworkSocket>,
}

pub(crate) struct NetworkServerPlayer {
    pub(crate) initialized: bool,
    pub(crate) handle: NetworkPlayer,
    pub(crate) socket: NetworkSocket,
}

pub struct NetworkServer {
    pub(crate) hosts: Vec<NetworkHost>,
    pub(crate) local_player: Option<NetworkPlayer>,
    pub(crate) joiners: Vec<NetworkServerJoiner>,
    pub(crate) players: Vec<NetworkServerPlayer>,
}

impl NetworkServer {
    pub(crate) fn new(hosts: Vec<NetworkHost>, local_player: Option<NetworkPlayer>) -> Self {
        Self {
            hosts,
            local_player,
            joiners: vec![],
            players: vec![],
        }
    }

    pub fn send_event(&mut self) {
        for player in self.players.iter_mut() {
            player.socket.send(NetworkMessage::Event.serialize());
        }
    }
}
