use crate::{
    events::NetworkEventTraits,
    messages::NetworkMessage,
    player::NetworkPlayer,
    serialized_struct::{NetworkSerializedStruct, NetworkSerializedStructMap},
};
use bevy_nety_protocol::{NetworkHost, NetworkSocket};

pub(crate) struct NetworkServerJoiner {
    pub(crate) socket: Option<NetworkSocket>,
}

pub(crate) struct NetworkServerPlayer {
    pub(crate) initialized: bool,
    pub(crate) handle: NetworkPlayer,
    pub(crate) socket: NetworkSocket,
    pub(crate) data: NetworkSerializedStructMap,
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

    pub fn send_to_all<T>(&mut self, event: T)
    where
        T: NetworkEventTraits,
    {
        for player in self.players.iter_mut() {
            player.socket.send(
                NetworkMessage::Event {
                    data: NetworkSerializedStruct::from_struct(&event),
                }
                .serialize(),
            );
        }
    }

    pub fn send_to_all_except_local<T>(&mut self, event: T)
    where
        T: NetworkEventTraits,
    {
        for player in self.players.iter_mut() {
            let is_local_player = if let Some(local_player) = self.local_player {
                player.handle == local_player
            } else {
                false
            };
            if !is_local_player {
                player.socket.send(
                    NetworkMessage::Event {
                        data: NetworkSerializedStruct::from_struct(&event),
                    }
                    .serialize(),
                );
            }
        }
    }

    pub fn send_to_players<T>(&mut self, players: &Vec<NetworkPlayer>, event: T)
    where
        T: NetworkEventTraits,
    {
        for player in self.players.iter_mut() {
            if players.contains(&player.handle) {
                player.socket.send(
                    NetworkMessage::Event {
                        data: NetworkSerializedStruct::from_struct(&event),
                    }
                    .serialize(),
                );
            }
        }
    }

    pub(crate) fn players(&self) -> Vec<NetworkPlayer> {
        self.players.iter().map(|p| p.handle).collect()
    }
}
