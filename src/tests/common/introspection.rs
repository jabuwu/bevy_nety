use super::test_structs::TestGameEvent;
use crate::{events::NetworkEntityEvent, prelude::*};
use bevy::prelude::*;

pub struct IntrospectionPlugin;

impl Plugin for IntrospectionPlugin {
    fn build(&self, app: &mut App) {
        // TODO: at what stage should capture_events run?
        app.init_resource::<Introspection>()
            .add_system(capture_events);
    }
}

#[derive(Default)]
pub struct Introspection {
    pub connect_events: Vec<NetworkConnectEvent>,
    pub connecting_events: Vec<NetworkConnectingEvent>,
    pub disconnect_events: Vec<NetworkDisconnectEvent>,
    pub player_join_events: Vec<NetworkPlayerJoinEvent>,
    pub player_leave_events: Vec<NetworkPlayerLeaveEvent>,
    pub test_game_events_on_client: Vec<NetworkEvent<TestGameEvent>>,
    pub test_game_events_on_server: Vec<NetworkServerEvent<TestGameEvent>>,
    pub test_entity_events: Vec<NetworkEntityEvent<TestGameEvent>>,
}

impl Introspection {
    pub fn clear(&mut self) {
        *self = Introspection::default();
    }
}

pub fn capture_events(
    mut introspection: ResMut<Introspection>,
    mut connect_events: EventReader<NetworkConnectEvent>,
    mut connecting_events: EventReader<NetworkConnectingEvent>,
    mut disconnect_events: EventReader<NetworkDisconnectEvent>,
    mut player_join_events: EventReader<NetworkPlayerJoinEvent>,
    mut player_leave_events: EventReader<NetworkPlayerLeaveEvent>,
    mut test_game_events_on_client: EventReader<NetworkEvent<TestGameEvent>>,
    mut test_game_events_on_server: EventReader<NetworkServerEvent<TestGameEvent>>,
    mut test_entity_events: EventReader<NetworkEntityEvent<TestGameEvent>>,
) {
    for event in connect_events.iter() {
        introspection.connect_events.push(event.clone());
    }
    for event in connecting_events.iter() {
        introspection.connecting_events.push(event.clone());
    }
    for event in disconnect_events.iter() {
        introspection.disconnect_events.push(event.clone());
    }
    for event in player_join_events.iter() {
        introspection.player_join_events.push(event.clone());
    }
    for event in player_leave_events.iter() {
        introspection.player_leave_events.push(event.clone());
    }
    for event in test_game_events_on_client.iter() {
        introspection.test_game_events_on_client.push(NetworkEvent {
            data: event.data.clone(),
        });
    }
    for event in test_game_events_on_server.iter() {
        introspection
            .test_game_events_on_server
            .push(NetworkServerEvent {
                from: event.from,
                data: event.data.clone(),
            });
    }
    for event in test_entity_events.iter() {
        introspection.test_entity_events.push(NetworkEntityEvent {
            entity: event.entity,
            from: event.from,
            data: event.data.clone(),
        });
    }
}
