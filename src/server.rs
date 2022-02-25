use crate::{
    entity::NetworkEntity,
    events::NetworkEventTraits,
    messages::NetworkMessage,
    player::NetworkPlayer,
    relevancy::NetworkRelevancy,
    serialized_struct::{NetworkSerializedStruct, NetworkSerializedStructMap},
};
use bevy_nety_protocol::{NetworkHost, NetworkSocket};
use std::collections::HashMap;

pub(crate) struct NetworkServerJoiner {
    pub(crate) socket: Option<NetworkSocket>,
}

pub(crate) struct NetworkServerPlayer {
    pub(crate) initialized: bool,
    pub(crate) handle: NetworkPlayer,
    pub(crate) socket: NetworkSocket,
    pub(crate) data: NetworkSerializedStructMap,
}

pub(crate) struct NetworkServerEntity {
    pub(crate) handle: NetworkEntity,
    pub(crate) exists: bool,
}

pub struct NetworkServer {
    pub(crate) hosts: Vec<NetworkHost>,
    pub(crate) local_player: Option<NetworkPlayer>,
    pub(crate) joiners: Vec<NetworkServerJoiner>,
    pub(crate) players: Vec<NetworkServerPlayer>,
    pub(crate) entities: HashMap<NetworkEntity, NetworkServerEntity>,
    pub(crate) relevancy: NetworkRelevancy,
}

impl NetworkServer {
    pub(crate) fn new(hosts: Vec<NetworkHost>, local_player: Option<NetworkPlayer>) -> Self {
        Self {
            hosts,
            local_player,
            joiners: vec![],
            players: vec![],
            entities: HashMap::new(),
            relevancy: NetworkRelevancy::default(),
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

    pub(crate) fn get_or_insert_entity(
        &mut self,
        entity: NetworkEntity,
    ) -> &mut NetworkServerEntity {
        self.entities
            .entry(entity)
            .or_insert_with(|| NetworkServerEntity {
                handle: entity,
                exists: true,
            })
    }

    pub fn set_entity_relevant(
        &mut self,
        entity: NetworkEntity,
        player: NetworkPlayer,
        relevant: bool,
    ) {
        self.relevancy.set_relevant(player, entity, relevant);
    }
}
