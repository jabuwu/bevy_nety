use bevy::prelude::*;

mod app_setup_for_tests;
mod introspection;
mod pseudo_network;
mod test_structs;

pub fn flush_network(mut apps: Vec<&mut App>) {
    for _ in 0..20 {
        for app in apps.iter_mut() {
            app.update();
        }
    }
}

pub fn flush_network_2(mut apps: Vec<&mut App>, apps2: &mut Vec<App>) {
    for _ in 0..20 {
        for app in apps.iter_mut() {
            app.update();
        }
        for app in apps2.iter_mut() {
            app.update();
        }
    }
}

pub mod prelude {
    pub use super::{
        app_setup_for_tests::AppSetupForTests,
        flush_network, flush_network_2,
        pseudo_network::{PseudoConnector, PseudoHost, PseudoNetwork},
        test_structs::{TestGameEvent, TestPlayerData},
    };
}
