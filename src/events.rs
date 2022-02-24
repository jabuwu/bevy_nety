use super::player::NetworkPlayer;
use bevy::ecs::system::Resource;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct NetworkConnectEvent {
    pub is_server: bool,
    pub is_client: bool,
}

#[derive(Clone, Debug)]
pub struct NetworkConnectingEvent;

#[derive(Clone, Debug)]
pub struct NetworkDisconnectEvent {
    pub failed_to_connect: bool,
}

#[derive(Debug, Clone)]
pub struct NetworkPlayerJoinEvent {
    pub player: NetworkPlayer,
    pub me: bool,
    pub existing_player: bool,
}

#[derive(Debug, Clone)]
pub struct NetworkPlayerLeaveEvent {
    pub player: NetworkPlayer,
}
pub struct NetworkEvent<T: Resource> {
    pub data: T,
}

pub struct NetworkServerEvent<T: Resource> {
    pub from: NetworkPlayer,
    pub data: T,
}

pub trait NetworkEventTraits: Resource + Serialize + DeserializeOwned {}
impl<T> NetworkEventTraits for T where T: Resource + Serialize + DeserializeOwned {}
