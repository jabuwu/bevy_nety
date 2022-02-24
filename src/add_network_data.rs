use super::events::{NetworkEvent, NetworkEventTraits, NetworkServerEvent};
use super::network::Network;
use bevy::prelude::*;

pub trait AddNetworkData {
    fn add_network_event<T>(&mut self) -> &mut Self
    where
        T: NetworkEventTraits;
}

impl AddNetworkData for App {
    fn add_network_event<T>(&mut self) -> &mut Self
    where
        T: NetworkEventTraits,
    {
        self.add_event::<NetworkEvent<T>>();
        self.add_event::<NetworkServerEvent<T>>();
        let mut network = self
            .world
            .get_resource_mut::<Network>()
            .expect("Can't register network event, please add the NetworkPlugin");
        network.registry.add_network_event::<T>();
        self
    }
}
