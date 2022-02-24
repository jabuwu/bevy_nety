use crate::{
    events::{NetworkEvent, NetworkEventTraits, NetworkServerEvent},
    player::NetworkPlayer,
    serialized_struct::NetworkSerializedStruct,
};
use bevy::{app::Events, prelude::*};
use std::{any::type_name, collections::HashMap};

// TODO: we use type_name as a key for networked classes,
//       probably worth making this configurable somehow

#[derive(Default)]
pub struct NetworkRegistryEntry {
    pub(crate) event: Option<NetworkRegistryEvent>,
}

impl NetworkRegistryEntry {
    pub fn get_or_insert_event<T>(&mut self) -> &mut NetworkRegistryEvent
    where
        T: NetworkEventTraits,
    {
        if self.event.is_none() {
            self.event = Some(NetworkRegistryEvent::new::<T>());
        }
        self.event.as_mut().unwrap()
    }
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

#[derive(Default)]
pub struct NetworkRegistry {
    entries: HashMap<String, NetworkRegistryEntry>,
}

impl NetworkRegistry {
    fn get_or_insert_entry(&mut self, key: &str) -> &mut NetworkRegistryEntry {
        self.entries
            .entry(String::from(key))
            .or_insert_with(|| NetworkRegistryEntry::default())
    }

    fn get_or_insert_event<T>(&mut self, key: &str) -> &mut NetworkRegistryEvent
    where
        T: NetworkEventTraits,
    {
        let entry = self.get_or_insert_entry(key);
        entry.get_or_insert_event::<T>()
    }

    pub fn add_network_event<T>(&mut self)
    where
        T: NetworkEventTraits,
    {
        self.get_or_insert_event::<T>(type_name::<T>());
    }

    pub fn get_entry(&mut self, s: &NetworkSerializedStruct) -> Option<&mut NetworkRegistryEntry> {
        self.entries.get_mut(&s.type_name)
    }
}
