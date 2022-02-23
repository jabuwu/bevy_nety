use bevy::prelude::*;
use bevy_nety::prelude::*;

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
    mut network_events: EventReader<NetworkEvent>,
) {
    for _ in connect_events.iter() {
        info!("Connected!");
    }
    for _ in disconnect_events.iter() {
        info!("Disconnected!");
    }
    for _ in network_events.iter() {
        info!("Got network event!");
    }
}

pub fn send_network_events(input: Res<Input<KeyCode>>, mut network: ResMut<Network>) {
    if input.just_pressed(KeyCode::Space) {
        network.server_mut().unwrap().send_event();
    }
}
