use crate::{
    events::{NetworkEvent, NetworkEventTraits, NetworkServerEvent},
    network::Network,
    player_data::NetworkPlayerDataTraits,
};
use bevy::prelude::*;

const ERROR_MESSAGE: &str = "Can't register network event, please add the NetworkPlugin";

pub trait AddNetworkData {
    fn add_network_event<T>(&mut self) -> &mut Self
    where
        T: NetworkEventTraits;

    fn add_network_player_data<T>(&mut self) -> &mut Self
    where
        T: NetworkPlayerDataTraits;
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
            .expect(ERROR_MESSAGE);
        network.registry.add_network_event::<T>();
        self
    }

    fn add_network_player_data<T>(&mut self) -> &mut Self
    where
        T: NetworkPlayerDataTraits,
    {
        let mut network = self
            .world
            .get_resource_mut::<Network>()
            .expect(ERROR_MESSAGE);
        network.registry.add_network_player_data::<T>();
        self
    }
}
