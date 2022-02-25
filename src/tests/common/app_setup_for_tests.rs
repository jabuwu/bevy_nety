use super::introspection::{Introspection, IntrospectionPlugin};
use super::test_structs::{TestGameEvent, TestPlayerData};
use crate::prelude::*;
use bevy::prelude::*;

pub trait AppSetupForTests {
    fn setup_for_tests(&mut self) -> &mut Self;
    fn network(&self) -> &Network;
    fn network_mut(&mut self) -> Mut<Network>;
    fn introspect(&self) -> &Introspection;
}

impl AppSetupForTests for App {
    fn setup_for_tests(&mut self) -> &mut Self {
        self.add_plugins(MinimalPlugins)
            .add_plugin(NetworkPlugin)
            .add_plugin(IntrospectionPlugin)
            .add_network_event::<TestGameEvent>()
            .add_network_entity_event::<TestGameEvent>()
            .add_network_player_data::<TestPlayerData>()
    }

    fn network(&self) -> &Network {
        self.world.get_resource::<Network>().unwrap()
    }

    fn network_mut(&mut self) -> Mut<Network> {
        self.world.get_resource_mut::<Network>().unwrap()
    }

    fn introspect(&self) -> &Introspection {
        self.world.get_resource::<Introspection>().unwrap()
    }
}
