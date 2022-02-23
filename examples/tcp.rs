use bevy::prelude::*;
use bevy_nety::prelude::*;
use bevy_nety_tcp::prelude::*;
use clap::{Parser, Subcommand};

/// Tcp bevy_nety example
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None, disable_help_subcommand = true)]
pub struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Host {
        /// Address to listen on, ex: 0.0.0.0:8000
        #[clap(short, long)]
        address: String,
    },
    Connect {
        /// Address to connect to, ex: 127.0.0.1:8000
        #[clap(short, long)]
        address: String,
    },
}

fn main() {
    let args = Args::parse();
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            width: 300.,
            height: 200.,
            ..Default::default()
        })
        .insert_resource(args)
        .add_plugins(DefaultPlugins)
        .add_plugin(NetworkPlugin)
        .add_startup_system(init)
        .add_system(network_events)
        .add_system(send_network_events)
        .run();
}

pub fn init(mut network: ResMut<Network>, args: Res<Args>) {
    match &args.command {
        Commands::Host { address } => {
            let host = TcpHost::listen(address).unwrap();
            network.start_server_client(vec![host]);
        }
        Commands::Connect { address } => {
            let connector = TcpConnector::connect(address);
            network.start_client(connector);
        }
    }
}

pub fn network_events(
    mut connecting_events: EventReader<NetworkConnectingEvent>,
    mut connect_events: EventReader<NetworkConnectEvent>,
    mut disconnect_events: EventReader<NetworkDisconnectEvent>,
    mut player_join_events: EventReader<NetworkPlayerJoinEvent>,
    mut player_leave_events: EventReader<NetworkPlayerLeaveEvent>,
    mut network_events: EventReader<NetworkEvent>,
) {
    for _ in connecting_events.iter() {
        info!("Connecting...");
    }
    for _ in connect_events.iter() {
        info!("Connected!");
    }
    for event in disconnect_events.iter() {
        if event.failed_to_connect {
            info!("Failed to connect!");
        } else {
            info!("Disconnected!");
        }
    }
    for _ in player_join_events.iter() {
        info!("Player joined!");
    }
    for _ in player_leave_events.iter() {
        info!("Player left!");
    }
    for _ in network_events.iter() {
        info!("Got network event!");
    }
}

pub fn send_network_events(input: Res<Input<KeyCode>>, mut network: ResMut<Network>) {
    if input.just_pressed(KeyCode::Space) {
        if let Some(server) = network.server_mut() {
            server.send_event();
        }
    }
}
