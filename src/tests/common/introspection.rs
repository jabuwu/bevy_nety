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
    pub disconnect_events: Vec<NetworkDisconnectEvent>,
    pub network_events: Vec<NetworkEvent>,
}

pub fn capture_events(
    mut introspection: ResMut<Introspection>,
    mut connect_events: EventReader<NetworkConnectEvent>,
    mut disconnect_events: EventReader<NetworkDisconnectEvent>,
    mut network_events: EventReader<NetworkEvent>,
) {
    for event in connect_events.iter() {
        introspection.connect_events.push(event.clone());
    }
    for event in disconnect_events.iter() {
        introspection.disconnect_events.push(event.clone());
    }
    for event in network_events.iter() {
        introspection.network_events.push(event.clone());
    }
}
