use super::messages::NetworkMessage;
use bevy_nety_protocol::{NetworkHost, NetworkSocket};

pub struct NetworkServer {
    hosts: Vec<NetworkHost>,
    sockets: Vec<NetworkSocket>,
}

impl NetworkServer {
    pub(crate) fn new(hosts: Vec<NetworkHost>) -> Self {
        Self {
            hosts,
            sockets: vec![],
        }
    }

    pub(crate) fn update(&mut self) {
        for host in self.hosts.iter_mut() {
            host.update();
            while let Some(socket) = host.accept() {
                self.sockets.push(socket);
            }
        }
    }

    pub fn send_event(&mut self) {
        for socket in self.sockets.iter_mut() {
            socket.send(NetworkMessage::Event.serialize());
        }
    }
}
