use crate::prelude::*;
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
    pub network_events: Vec<NetworkEvent>,
}

pub fn capture_events(
    mut introspection: ResMut<Introspection>,
    mut connect_events: EventReader<NetworkConnectEvent>,
    mut connecting_events: EventReader<NetworkConnectingEvent>,
    mut disconnect_events: EventReader<NetworkDisconnectEvent>,
    mut player_join_events: EventReader<NetworkPlayerJoinEvent>,
    mut player_leave_events: EventReader<NetworkPlayerLeaveEvent>,
    mut network_events: EventReader<NetworkEvent>,
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
    for event in network_events.iter() {
        introspection.network_events.push(event.clone());
    }
}
