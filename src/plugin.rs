use crate::{
    events::{
        NetworkConnectEvent, NetworkConnectingEvent, NetworkDisconnectEvent,
        NetworkPlayerJoinEvent, NetworkPlayerLeaveEvent,
    },
    network::{update_network, Network},
};
use bevy::prelude::*;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        // TODO: what stage should network run? first? last?
        app.init_resource::<Network>()
            .add_event::<NetworkConnectEvent>()
            .add_event::<NetworkConnectingEvent>()
            .add_event::<NetworkDisconnectEvent>()
            .add_event::<NetworkPlayerJoinEvent>()
            .add_event::<NetworkPlayerLeaveEvent>()
            .add_system(update_network.exclusive_system());
    }
}
