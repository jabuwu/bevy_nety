use bevy::prelude::*;
use bevy_nety::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameEvent {
    message: String,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            width: 300.,
            height: 200.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(NetworkPlugin)
        .add_network_event::<GameEvent>()
        .add_startup_system(init)
        .add_system(network_events)
        .add_system(send_network_events)
        .run();
}

pub fn init(mut network: ResMut<Network>) {
    network.start_local();
}

pub fn network_events(
    mut connect_events: EventReader<NetworkConnectEvent>,
    mut disconnect_events: EventReader<NetworkDisconnectEvent>,
    mut game_events: EventReader<NetworkEvent<GameEvent>>,
) {
    for _ in connect_events.iter() {
        info!("Connected!");
    }
    for _ in disconnect_events.iter() {
        info!("Disconnected!");
    }
    for event in game_events.iter() {
        info!("Game Event: {}", event.data.message);
    }
}

pub fn send_network_events(input: Res<Input<KeyCode>>, mut network: ResMut<Network>) {
    if input.just_pressed(KeyCode::Space) {
        if let Some(server) = network.server_mut() {
            server.send_to_all(GameEvent {
                message: "hello world".into(),
            });
        }
    }
}
