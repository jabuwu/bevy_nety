use crate::{
    entity::NetworkEntity,
    events::{
        NetworkConnectEvent, NetworkConnectingEvent, NetworkDisconnectEvent,
        NetworkPlayerJoinEvent, NetworkPlayerLeaveEvent,
    },
    player::NetworkPlayer,
    registry::NetworkRegistry,
    serialized_struct::NetworkSerializedStruct,
};
use bevy::{app::Events, prelude::*};
use std::collections::VecDeque;

#[derive(Default)]
pub(crate) struct EventQueue {
    connect_events: VecDeque<NetworkConnectEvent>,
    connecting_events: VecDeque<NetworkConnectingEvent>,
    disconnect_events: VecDeque<NetworkDisconnectEvent>,
    player_join_events: VecDeque<NetworkPlayerJoinEvent>,
    player_leave_events: VecDeque<NetworkPlayerLeaveEvent>,
    network_events: VecDeque<NetworkSerializedStruct>,
    network_server_events: VecDeque<(NetworkPlayer, NetworkSerializedStruct)>,
    network_entity_events: VecDeque<(
        NetworkEntity,
        Option<NetworkPlayer>,
        NetworkSerializedStruct,
    )>,
}

impl EventQueue {
    pub(crate) fn connect(&mut self, event: NetworkConnectEvent) {
        self.connect_events.push_back(event);
    }

    pub(crate) fn connecting(&mut self, event: NetworkConnectingEvent) {
        self.connecting_events.push_back(event);
    }

    pub(crate) fn disconnect(&mut self, event: NetworkDisconnectEvent) {
        self.disconnect_events.push_back(event);
    }

    pub(crate) fn player_join(&mut self, event: NetworkPlayerJoinEvent) {
        self.player_join_events.push_back(event);
    }

    pub(crate) fn player_leave(&mut self, event: NetworkPlayerLeaveEvent) {
        self.player_leave_events.push_back(event);
    }

    pub(crate) fn network(&mut self, event: NetworkSerializedStruct) {
        self.network_events.push_back(event);
    }

    pub(crate) fn network_server(&mut self, from: NetworkPlayer, event: NetworkSerializedStruct) {
        self.network_server_events.push_back((from, event));
    }

    pub(crate) fn network_entity(
        &mut self,
        entity: NetworkEntity,
        from: Option<NetworkPlayer>,
        event: NetworkSerializedStruct,
    ) {
        self.network_entity_events.push_back((entity, from, event));
    }

    pub(crate) fn send_to_world(&mut self, world: &mut World, registry: &mut NetworkRegistry) {
        while let Some(connect_event) = self.connect_events.pop_front() {
            let mut events = world
                .get_resource_mut::<Events<NetworkConnectEvent>>()
                .unwrap();
            events.send(connect_event);
        }
        while let Some(connecting_event) = self.connecting_events.pop_front() {
            let mut events = world
                .get_resource_mut::<Events<NetworkConnectingEvent>>()
                .unwrap();
            events.send(connecting_event);
        }
        while let Some(disconnect_event) = self.disconnect_events.pop_front() {
            let mut events = world
                .get_resource_mut::<Events<NetworkDisconnectEvent>>()
                .unwrap();
            events.send(disconnect_event);
        }
        while let Some(player_join_event) = self.player_join_events.pop_front() {
            let mut events = world
                .get_resource_mut::<Events<NetworkPlayerJoinEvent>>()
                .unwrap();
            events.send(player_join_event);
        }
        while let Some(player_leave_event) = self.player_leave_events.pop_front() {
            let mut events = world
                .get_resource_mut::<Events<NetworkPlayerLeaveEvent>>()
                .unwrap();
            events.send(player_leave_event);
        }
        while let Some(network_event) = self.network_events.pop_front() {
            if let Some(entry) = registry.get_entry_from_serialized(&network_event) {
                if let Some(event) = &mut entry.event {
                    (event.send_to_world)(world, network_event);
                }
            }
        }
        while let Some((from, network_server_event)) = self.network_server_events.pop_front() {
            if let Some(entry) = registry.get_entry_from_serialized(&network_server_event) {
                if let Some(event) = &mut entry.event {
                    (event.send_to_server_world)(world, from, network_server_event);
                }
            }
        }
        let mut network_entity_query = world.query::<(Entity, &NetworkEntity)>();
        while let Some((network_entity, from, network_entity_event)) =
            self.network_entity_events.pop_front()
        {
            if let Some(entry) = registry.get_entry_from_serialized(&network_entity_event) {
                if let Some(event) = &mut entry.entity_event {
                    let entity = network_entity_query
                        .iter(world)
                        .find(|(_, ne)| **ne == network_entity);
                    if let Some((entity, _)) = entity {
                        (event.send_to_world)(world, entity, from, network_entity_event);
                    }
                }
            }
        }
    }
}
