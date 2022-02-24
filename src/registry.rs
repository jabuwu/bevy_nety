use crate::{
    events::{NetworkEvent, NetworkEventTraits, NetworkServerEvent},
    network_type_name::NetworkTypeName,
    player::NetworkPlayer,
    player_data::NetworkPlayerDataTraits,
    serialized_struct::NetworkSerializedStruct,
};
use bevy::{app::Events, prelude::*};
use std::collections::HashMap;

#[derive(Default)]
pub struct NetworkRegistryEntry {
    pub(crate) event: Option<NetworkRegistryEvent>,
    pub(crate) player_data: Option<NetworkRegistryPlayerData>,
}

pub struct NetworkRegistryEvent {
    pub(crate) send_to_world: Box<dyn Fn(&mut World, NetworkSerializedStruct) + Send + Sync>,
    pub(crate) send_to_server_world:
        Box<dyn Fn(&mut World, NetworkPlayer, NetworkSerializedStruct) + Send + Sync>,
}

impl NetworkRegistryEvent {
    fn new<T>() -> Self
    where
        T: NetworkEventTraits,
    {
        Self {
            send_to_world: Box::new(|world: &mut World, s: NetworkSerializedStruct| {
                let mut events = world.get_resource_mut::<Events<NetworkEvent<T>>>().unwrap();
                events.send(NetworkEvent {
                    data: s.to_struct::<T>().unwrap(),
                });
            }),
            send_to_server_world: Box::new(
                |world: &mut World, from: NetworkPlayer, s: NetworkSerializedStruct| {
                    let mut events = world
                        .get_resource_mut::<Events<NetworkServerEvent<T>>>()
                        .unwrap();
                    events.send(NetworkServerEvent {
                        from,
                        data: s.to_struct::<T>().unwrap(),
                    });
                },
            ),
        }
    }
}

pub struct NetworkRegistryPlayerData {}

impl NetworkRegistryPlayerData {
    fn new<T>() -> Self
    where
        T: NetworkPlayerDataTraits,
    {
        Self {}
    }
}

#[derive(Default)]
pub struct NetworkRegistry {
    entries: HashMap<NetworkTypeName, NetworkRegistryEntry>,
}

impl NetworkRegistry {
    fn get_or_insert_entry(&mut self, type_name: NetworkTypeName) -> &mut NetworkRegistryEntry {
        self.entries
            .entry(type_name)
            .or_insert_with(|| NetworkRegistryEntry::default())
    }

    fn get_or_insert_event<T>(&mut self, type_name: NetworkTypeName) -> &mut NetworkRegistryEvent
    where
        T: NetworkEventTraits,
    {
        let entry = self.get_or_insert_entry(type_name);
        if entry.event.is_none() {
            entry.event = Some(NetworkRegistryEvent::new::<T>());
        }
        entry.event.as_mut().unwrap()
    }

    fn get_or_insert_player_data<T>(
        &mut self,
        type_name: NetworkTypeName,
    ) -> &mut NetworkRegistryPlayerData
    where
        T: NetworkPlayerDataTraits,
    {
        let entry = self.get_or_insert_entry(type_name);
        if entry.player_data.is_none() {
            entry.player_data = Some(NetworkRegistryPlayerData::new::<T>());
        }
        entry.player_data.as_mut().unwrap()
    }

    pub fn add_network_event<T>(&mut self)
    where
        T: NetworkEventTraits,
    {
        self.get_or_insert_event::<T>(NetworkTypeName::of::<T>());
    }

    pub fn add_network_player_data<T>(&mut self)
    where
        T: NetworkPlayerDataTraits,
    {
        self.get_or_insert_player_data::<T>(NetworkTypeName::of::<T>());
    }

    pub fn get_entry<T>(&mut self) -> Option<&mut NetworkRegistryEntry> {
        self.entries.get_mut(&NetworkTypeName::of::<T>())
    }

    pub fn get_entry_from_serialized(
        &mut self,
        s: &NetworkSerializedStruct,
    ) -> Option<&mut NetworkRegistryEntry> {
        self.entries.get_mut(&s.type_name)
    }
}
