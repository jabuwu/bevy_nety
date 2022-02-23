use super::events::NetworkEvent;
use super::messages::NetworkMessage;
use bevy::app::Events;
use bevy::prelude::*;
use bevy_nety_protocol::NetworkSocket;
use std::collections::VecDeque;

pub struct NetworkClient {
    socket: NetworkSocket,
    events: VecDeque<NetworkEvent>,
}

impl NetworkClient {
    pub(crate) fn new(socket: NetworkSocket) -> Self {
        Self {
            socket,
            events: VecDeque::new(),
        }
    }

    pub(crate) fn update(&mut self) {
        self.socket.update();
        while let Some(message) = self.socket.receive() {
            let message = NetworkMessage::deserialize(&message);
            match message {
                NetworkMessage::Event => {
                    self.events.push_back(NetworkEvent);
                }
            }
        }
    }

    pub(crate) fn update_world(&mut self, world: &mut World) {
        if !self.events.is_empty() {
            while let Some(event) = self.events.pop_front() {
                let mut world_events = world.get_resource_mut::<Events<NetworkEvent>>().unwrap();
                world_events.send(event);
            }
        }
    }
}
