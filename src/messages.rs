use crate::{
    entity::NetworkEntity,
    player::NetworkPlayer,
    serialized_struct::{NetworkSerializedStruct, NetworkSerializedStructMap},
    serializer::{deserialize, serialize},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NetworkMessage {
    PlayerInit {
        player: NetworkPlayer,
        data: NetworkSerializedStructMap,
    },
    PlayerJoin {
        player: NetworkPlayer,
        me: bool,
        data: NetworkSerializedStructMap,
    },
    PlayerLeave {
        player: NetworkPlayer,
    },
    Event {
        data: NetworkSerializedStruct,
    },
    EntitySpawn {
        entity: NetworkEntity,
    },
    EntityDespawn {
        entity: NetworkEntity,
    },
    EntityOwner {
        entity: NetworkEntity,
        owner: bool,
    },
    EntityEvent {
        entity: NetworkEntity,
        from: Option<NetworkPlayer>,
        data: NetworkSerializedStruct,
    },
}

impl NetworkMessage {
    pub fn deserialize(string: &str) -> Self {
        deserialize::<Self>(string)
    }
    pub fn serialize(&self) -> String {
        serialize(&self)
    }
}
