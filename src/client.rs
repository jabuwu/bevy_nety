use crate::{
    entity::NetworkEntity,
    events::NetworkEventTraits,
    messages::NetworkMessage,
    player::NetworkPlayer,
    serialized_struct::{NetworkSerializedStruct, NetworkSerializedStructMap},
};
use bevy::prelude::*;
use bevy_nety_protocol::NetworkSocket;
use std::collections::HashMap;

pub(crate) struct NetworkClientPlayer {
    pub(crate) handle: NetworkPlayer,
    pub(crate) data: NetworkSerializedStructMap,
}

pub(crate) struct NetworkClientEntity {
    pub(crate) initialized: bool,
    pub(crate) exists: bool,
    pub(crate) local_entity: Option<Entity>,
    pub(crate) owner: bool,
}

pub struct NetworkClient {
    pub(crate) initialized: bool,
    pub(crate) socket: NetworkSocket,
    pub(crate) me: NetworkPlayer,
    pub(crate) players: Vec<NetworkClientPlayer>,
    pub(crate) existing_player_flag: bool,
    pub(crate) entities: HashMap<NetworkEntity, NetworkClientEntity>,
}

impl NetworkClient {
    pub(crate) fn new(socket: NetworkSocket, me: NetworkPlayer) -> Self {
        Self {
            initialized: false,
            socket,
            me,
            players: vec![],
            existing_player_flag: true,
            entities: HashMap::new(),
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

    pub fn send_to_entity<T>(&mut self, entity: NetworkEntity, event: T)
    where
        T: NetworkEventTraits,
    {
        self.socket.send(
            NetworkMessage::EntityEvent {
                entity,
                from: Some(self.me),
                data: NetworkSerializedStruct::from_struct(&event),
            }
            .serialize(),
        );
    }

    pub(crate) fn players(&self) -> Vec<NetworkPlayer> {
        self.players.iter().map(|p| p.handle).collect()
    }

    pub(crate) fn is_entity_owner(&mut self, entity: NetworkEntity) -> bool {
        if let Some(entity) = self.entities.get(&entity) {
            entity.owner
        } else {
            false
        }
    }
}
