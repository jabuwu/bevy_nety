use crate::{
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
pub struct EventQueue {
    connect_events: VecDeque<NetworkConnectEvent>,
    connecting_events: VecDeque<NetworkConnectingEvent>,
    disconnect_events: VecDeque<NetworkDisconnectEvent>,
    player_join_events: VecDeque<NetworkPlayerJoinEvent>,
    player_leave_events: VecDeque<NetworkPlayerLeaveEvent>,
    network_events: VecDeque<NetworkSerializedStruct>,
    network_server_events: VecDeque<(NetworkPlayer, NetworkSerializedStruct)>,
}

impl EventQueue {
    pub fn connect(&mut self, event: NetworkConnectEvent) {
        self.connect_events.push_back(event);
    }

    pub fn connecting(&mut self, event: NetworkConnectingEvent) {
        self.connecting_events.push_back(event);
    }

    pub fn disconnect(&mut self, event: NetworkDisconnectEvent) {
        self.disconnect_events.push_back(event);
    }

    pub fn player_join(&mut self, event: NetworkPlayerJoinEvent) {
        self.player_join_events.push_back(event);
    }

    pub fn player_leave(&mut self, event: NetworkPlayerLeaveEvent) {
        self.player_leave_events.push_back(event);
    }

    pub fn network(&mut self, event: NetworkSerializedStruct) {
        self.network_events.push_back(event);
    }

    pub fn network_server(&mut self, from: NetworkPlayer, event: NetworkSerializedStruct) {
        self.network_server_events.push_back((from, event));
    }

    pub fn send_to_world(&mut self, world: &mut World, registry: &mut NetworkRegistry) {
        if let Some(connect_event) = self.connect_events.pop_front() {
            let mut events = world
                .get_resource_mut::<Events<NetworkConnectEvent>>()
                .unwrap();
            events.send(connect_event);
        }
        if let Some(connecting_event) = self.connecting_events.pop_front() {
            let mut events = world
                .get_resource_mut::<Events<NetworkConnectingEvent>>()
                .unwrap();
            events.send(connecting_event);
        }
        if let Some(disconnect_event) = self.disconnect_events.pop_front() {
            let mut events = world
                .get_resource_mut::<Events<NetworkDisconnectEvent>>()
                .unwrap();
            events.send(disconnect_event);
        }
        if let Some(player_join_event) = self.player_join_events.pop_front() {
            let mut events = world
                .get_resource_mut::<Events<NetworkPlayerJoinEvent>>()
                .unwrap();
            events.send(player_join_event);
        }
        if let Some(player_leave_event) = self.player_leave_events.pop_front() {
            let mut events = world
                .get_resource_mut::<Events<NetworkPlayerLeaveEvent>>()
                .unwrap();
            events.send(player_leave_event);
        }
        if let Some(network_event) = self.network_events.pop_front() {
            if let Some(entry) = registry.get_entry_from_serialized(&network_event) {
                if let Some(event) = &mut entry.event {
                    (event.send_to_world)(world, network_event);
                }
            }
        }
        if let Some((from, network_server_event)) = self.network_server_events.pop_front() {
            if let Some(entry) = registry.get_entry_from_serialized(&network_server_event) {
                if let Some(event) = &mut entry.event {
                    (event.send_to_server_world)(world, from, network_server_event);
                }
            }
        }
    }
}
