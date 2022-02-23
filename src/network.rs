use super::client::NetworkClient;
use super::events::{NetworkConnectEvent, NetworkDisconnectEvent};
use super::internal_protocol::InternalHost;
use super::server::NetworkServer;
use bevy::app::Events;
use bevy::prelude::*;

pub enum NetworkState {
    Connected {
        server: Option<NetworkServer>,
        client: Option<NetworkClient>,
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
    connect_event: Option<NetworkConnectEvent>,
    disconnect_event: Option<NetworkDisconnectEvent>,
}

impl Network {
    pub fn start_local(&mut self) {
        let (host, socket) = InternalHost::new_pair();
        self.state = NetworkState::Connected {
            server: Some(NetworkServer::new(vec![host])),
            client: Some(NetworkClient::new(socket)),
        };
        self.connect_event = Some(NetworkConnectEvent);
    }

    pub fn stop(&mut self) {
        self.state = NetworkState::Disconnected;
        self.disconnect_event = Some(NetworkDisconnectEvent);
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

    pub fn is_disconnected(&mut self) -> bool {
        match &self.state {
            NetworkState::Connected { .. } => false,
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
}

pub fn update_network(world: &mut World) {
    let unsafe_world = unsafe { &mut *(world as *mut World) };
    let mut network = unsafe_world.get_resource_mut::<Network>().unwrap();
    if let Some(connect_event) = network.connect_event.take() {
        let mut events = world
            .get_resource_mut::<Events<NetworkConnectEvent>>()
            .unwrap();
        events.send(connect_event);
    }
    if let Some(disconnect_event) = network.disconnect_event.take() {
        let mut events = world
            .get_resource_mut::<Events<NetworkDisconnectEvent>>()
            .unwrap();
        events.send(disconnect_event);
    }
    if let Some(client) = network.client_mut() {
        client.update();
        client.update_world(world);
    }
    if let Some(server) = network.server_mut() {
        server.update();
    }
}
