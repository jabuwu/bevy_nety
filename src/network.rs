use crate::{
    client::{NetworkClient, NetworkClientPlayer},
    event_queue::EventQueue,
    events::{
        NetworkConnectEvent, NetworkConnectingEvent, NetworkDisconnectEvent,
        NetworkPlayerJoinEvent, NetworkPlayerLeaveEvent,
    },
    internal_protocol::InternalHost,
    messages::NetworkMessage,
    player::NetworkPlayer,
    player_data::NetworkPlayerDataTraits,
    registry::NetworkRegistry,
    serialized_struct::NetworkSerializedStructMap,
    server::{NetworkServer, NetworkServerJoiner, NetworkServerPlayer},
};
use bevy::prelude::*;
use bevy_nety_protocol::{NetworkConnectStatus, NetworkConnector, NetworkHost};
use std::any::type_name;

pub enum NetworkState {
    Connected {
        server: Option<NetworkServer>,
        client: Option<NetworkClient>,
    },
    Connecting {
        connector: NetworkConnector,
    },
    Disconnected,
}

impl Default for NetworkState {
    fn default() -> Self {
        Self::Disconnected
    }
}

#[derive(Default)]
pub struct Network {
    state: NetworkState,
    event_queue: EventQueue,
    pub(crate) registry: NetworkRegistry,
    my_player_data: NetworkSerializedStructMap,
}

impl Network {
    pub fn start_local(&mut self) {
        let (host, socket) = InternalHost::new_pair();
        let local_player = NetworkPlayer::new();
        self.state = NetworkState::Connected {
            server: Some(NetworkServer::new(vec![host], Some(local_player))),
            client: Some(NetworkClient::new(socket, local_player)),
        };
        self.event_queue.connect(NetworkConnectEvent {
            is_server: true,
            is_client: true,
        });
    }

    pub fn start_server_client(&mut self, mut hosts: Vec<NetworkHost>) {
        let (host, socket) = InternalHost::new_pair();
        let local_player = NetworkPlayer::new();
        hosts.push(host);
        self.state = NetworkState::Connected {
            server: Some(NetworkServer::new(hosts, Some(local_player))),
            client: Some(NetworkClient::new(socket, local_player)),
        };
        self.event_queue.connect(NetworkConnectEvent {
            is_server: true,
            is_client: true,
        });
    }

    pub fn start_server(&mut self, hosts: Vec<NetworkHost>) {
        self.state = NetworkState::Connected {
            server: Some(NetworkServer::new(hosts, None)),
            client: None,
        };
        self.event_queue.connect(NetworkConnectEvent {
            is_server: true,
            is_client: false,
        });
    }

    pub fn start_client(&mut self, connector: NetworkConnector) {
        self.state = NetworkState::Connecting { connector };
        self.event_queue.connecting(NetworkConnectingEvent);
    }

    pub fn stop(&mut self) {
        self.state = NetworkState::Disconnected;
        self.event_queue.disconnect(NetworkDisconnectEvent {
            failed_to_connect: false,
        });
    }

    pub fn is_server(&mut self) -> bool {
        match &self.state {
            NetworkState::Connected { server, .. } => server.is_some(),
            _ => false,
        }
    }

    pub fn is_client(&mut self) -> bool {
        match &self.state {
            NetworkState::Connected { client, .. } => client.is_some(),
            _ => false,
        }
    }

    pub fn is_connected(&mut self) -> bool {
        match &self.state {
            NetworkState::Connected { .. } => true,
            _ => false,
        }
    }

    pub fn is_connecting(&mut self) -> bool {
        match &self.state {
            NetworkState::Connecting { .. } => true,
            _ => false,
        }
    }

    pub fn is_disconnected(&mut self) -> bool {
        match &self.state {
            NetworkState::Connected { .. } => false,
            NetworkState::Connecting { .. } => false,
            _ => true,
        }
    }

    pub fn server(&self) -> Option<&NetworkServer> {
        match &self.state {
            NetworkState::Connected { server, .. } => server.as_ref(),
            _ => None,
        }
    }

    pub fn server_mut(&mut self) -> Option<&mut NetworkServer> {
        match &mut self.state {
            NetworkState::Connected { server, .. } => server.as_mut(),
            _ => None,
        }
    }

    pub fn client(&self) -> Option<&NetworkClient> {
        match &self.state {
            NetworkState::Connected { client, .. } => client.as_ref(),
            _ => None,
        }
    }

    pub fn client_mut(&mut self) -> Option<&mut NetworkClient> {
        match &mut self.state {
            NetworkState::Connected { client, .. } => client.as_mut(),
            _ => None,
        }
    }

    pub fn me(&self) -> Option<NetworkPlayer> {
        if let Some(client) = self.client() {
            Some(client.me)
        } else {
            None
        }
    }

    pub fn players(&self) -> Vec<NetworkPlayer> {
        match &self.state {
            NetworkState::Connected { server, client } => {
                if let Some(server) = server {
                    server.players()
                } else if let Some(client) = client {
                    client.players()
                } else {
                    vec![]
                }
            }
            _ => vec![],
        }
    }

    pub fn set_my_player_data<T>(&mut self, data: T)
    where
        T: NetworkPlayerDataTraits,
    {
        if let Some(entry) = self.registry.get_entry::<T>() {
            if let None = &entry.player_data {
                panic!(
                    "The struct \"{}\" has not been registered as networked player data.",
                    type_name::<T>()
                );
            }
        } else {
            panic!(
                "The struct \"{}\" has not been registered as networked player data.",
                type_name::<T>()
            );
        }
        match &self.state {
            NetworkState::Connected { .. } => {
                // TODO: probably shouldn't be a panic
                //       either allow this behavior or switch to error message
                panic!("Cannot set player data while connected.");
            }
            NetworkState::Connecting { .. } => {
                // TODO: probably shouldn't be a panic
                //       either allow this behavior or switch to error message
                panic!("Cannot set player data while connecting.");
            }
            _ => {
                self.my_player_data.set(data);
            }
        }
    }

    pub fn get_player_data<T>(&self, player: NetworkPlayer) -> T
    where
        T: NetworkPlayerDataTraits,
    {
        match &self.state {
            NetworkState::Connected { server, client } => {
                if let Some(server) = server {
                    if let Some(player) = server.players.iter().find(|p| p.handle == player) {
                        if let Some(data) = player.data.get::<T>() {
                            data
                        } else {
                            T::default()
                        }
                    } else {
                        T::default()
                    }
                } else if let Some(client) = client {
                    if let Some(player) = client.players.iter().find(|p| p.handle == player) {
                        if let Some(data) = player.data.get::<T>() {
                            data
                        } else {
                            T::default()
                        }
                    } else {
                        T::default()
                    }
                } else {
                    T::default()
                }
            }
            NetworkState::Connecting { .. } => T::default(),
            _ => T::default(),
        }
    }
}

macro_rules! get_server_from_state {
    ($x:expr) => {{
        if let NetworkState::Connected { server, .. } = $x {
            if let Some(client) = server {
                client
            } else {
                return;
            }
        } else {
            return;
        }
    }};
}

macro_rules! get_client_from_state {
    ($x:expr) => {{
        if let NetworkState::Connected { client, .. } = $x {
            if let Some(client) = client {
                client
            } else {
                return;
            }
        } else {
            return;
        }
    }};
}

// TODO: there's no reason this needs to be an exclusive uber system
//       it's just easier for now
pub fn update_network(world: &mut World) {
    let unsafe_world = unsafe { &mut *(world as *mut World) };
    let mut network = unsafe_world.get_resource_mut::<Network>().unwrap();
    update_connector(&mut network);
    client_initialize(&mut network);
    server_accept_sockets(&mut network);
    client_receive_messages(&mut network);
    server_receive_messages_from_joiners(&mut network);
    server_initialize_players(&mut network);
    server_receive_messages_from_players(&mut network);
    client_check_disconnect(&mut network);
    server_check_disconnects(&mut network);
    send_events(&mut network, world);
}

fn update_connector(mut network: &mut Network) {
    let Network {
        state, event_queue, ..
    } = &mut network;
    if let NetworkState::Connecting { connector } = state {
        match connector.status() {
            NetworkConnectStatus::Connected(socket) => {
                event_queue.connect(NetworkConnectEvent {
                    is_server: false,
                    is_client: true,
                });
                let local_player = NetworkPlayer::new();
                *state = NetworkState::Connected {
                    server: None,
                    client: Some(NetworkClient::new(socket, local_player)),
                };
            }
            NetworkConnectStatus::Connecting => {}
            NetworkConnectStatus::Failed => {
                event_queue.disconnect(NetworkDisconnectEvent {
                    failed_to_connect: true,
                });
                *state = NetworkState::Disconnected;
            }
        }
    }
}

fn client_initialize(network: &mut Network) {
    let Network {
        state,
        my_player_data,
        ..
    } = network;
    let client = get_client_from_state!(state);
    if !client.initialized {
        client.socket.send(
            NetworkMessage::PlayerInit {
                player: client.me,
                data: my_player_data.clone(),
            }
            .serialize(),
        );
        client.initialized = true;
    }
}

pub fn server_accept_sockets(network: &mut Network) {
    let Network { state, .. } = network;
    let server = get_server_from_state!(state);
    for host in server.hosts.iter_mut() {
        host.update();
        while let Some(socket) = host.accept() {
            server.joiners.push(NetworkServerJoiner {
                socket: Some(socket),
            });
        }
    }
}

pub fn client_receive_messages(network: &mut Network) {
    let Network {
        state, event_queue, ..
    } = network;
    let client = get_client_from_state!(state);
    client.socket.update();
    while let Some(message) = client.socket.receive() {
        let message = NetworkMessage::deserialize(&message);
        match message {
            NetworkMessage::PlayerJoin { player, me, data } => {
                if me {
                    client.existing_player_flag = false;
                }
                client.players.push(NetworkClientPlayer {
                    handle: player,
                    data,
                });
                event_queue.player_join(NetworkPlayerJoinEvent {
                    player,
                    me,
                    existing_player: client.existing_player_flag,
                });
            }
            NetworkMessage::PlayerLeave { player } => {
                client.players.retain(|p| p.handle != player);
                event_queue.player_leave(NetworkPlayerLeaveEvent { player });
            }
            NetworkMessage::Event { data } => {
                event_queue.network(data);
            }
            _ => {
                // TODO: disconnect for bad data?
            }
        }
    }
}

pub fn server_receive_messages_from_joiners(network: &mut Network) {
    let Network { state, .. } = network;
    let server = get_server_from_state!(state);
    for joiner in server.joiners.iter_mut() {
        if let Some(socket) = &mut joiner.socket {
            // TODO: check for joiner disconnects (+ tests)
            socket.update();
            while let Some(message) = socket.receive() {
                let message = NetworkMessage::deserialize(&message);
                match message {
                    NetworkMessage::PlayerInit { player, data } => {
                        server.players.push(NetworkServerPlayer {
                            initialized: false,
                            handle: player,
                            socket: joiner.socket.take().unwrap(),
                            data,
                        });
                        break;
                    }
                    _ => {
                        // TODO: disconnect for bad data?
                    }
                }
            }
        }
    }
    server.joiners.retain(|joiner| joiner.socket.is_some());
}

pub fn server_initialize_players(network: &mut Network) {
    let Network {
        state, event_queue, ..
    } = network;
    let server = get_server_from_state!(state);
    let unsafe_players = unsafe { &mut *(&mut server.players as *mut Vec<NetworkServerPlayer>) };
    for player in server.players.iter_mut() {
        if !player.initialized {
            if server.local_player.is_none() {
                event_queue.player_join(NetworkPlayerJoinEvent {
                    player: player.handle,
                    me: false,
                    existing_player: false,
                });
            }
            for other_player in unsafe_players.iter_mut() {
                let me = player.handle == other_player.handle;
                if other_player.initialized || me {
                    player.socket.send(
                        NetworkMessage::PlayerJoin {
                            player: other_player.handle,
                            me,
                            data: other_player.data.clone(),
                        }
                        .serialize(),
                    );
                    if !me {
                        other_player.socket.send(
                            NetworkMessage::PlayerJoin {
                                player: player.handle,
                                me: false,
                                data: player.data.clone(),
                            }
                            .serialize(),
                        );
                    }
                }
            }
            player.initialized = true;
        }
    }
}

pub fn server_receive_messages_from_players(network: &mut Network) {
    let Network {
        state, event_queue, ..
    } = network;
    let server = get_server_from_state!(state);
    for player in server.players.iter_mut() {
        player.socket.update();
        if let Some(message) = player.socket.receive() {
            let message = NetworkMessage::deserialize(&message);
            match message {
                NetworkMessage::Event { data } => {
                    event_queue.network_server(player.handle, data);
                }
                _ => {
                    // TODO: disconnect for bad data?
                }
            }
        }
    }
}

pub fn client_check_disconnect(network: &mut Network) {
    let Network {
        state, event_queue, ..
    } = network;
    let client = get_client_from_state!(state);
    if !client.socket.connected() {
        *state = NetworkState::Disconnected;
        event_queue.disconnect(NetworkDisconnectEvent {
            failed_to_connect: false,
        });
    }
}

pub fn server_check_disconnects(network: &mut Network) {
    let Network {
        state, event_queue, ..
    } = network;
    let server = get_server_from_state!(state);
    let mut disconnected_players = vec![];
    for player in server.players.iter_mut() {
        if !player.socket.connected() {
            disconnected_players.push(player.handle);
        }
    }
    server
        .players
        .retain(|p| !disconnected_players.contains(&p.handle));
    for disconnected_player in disconnected_players.iter() {
        if server.local_player.is_none() {
            event_queue.player_leave(NetworkPlayerLeaveEvent {
                player: *disconnected_player,
            });
        }
        for player in server.players.iter_mut() {
            player.socket.send(
                NetworkMessage::PlayerLeave {
                    player: *disconnected_player,
                }
                .serialize(),
            );
        }
    }
}

fn send_events(network: &mut Network, world: &mut World) {
    let Network {
        registry,
        event_queue,
        ..
    } = network;
    event_queue.send_to_world(world, registry);
}
