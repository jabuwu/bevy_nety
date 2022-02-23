use bevy::prelude::*;

mod app_setup_for_tests;
mod introspection;
mod pseudo_network;

pub fn flush_network(mut apps: Vec<&mut App>) {
    for _ in 0..10 {
        for app in apps.iter_mut() {
            app.update();
        }
    }
}

pub mod prelude {
    pub use super::{
        app_setup_for_tests::AppSetupForTests,
        flush_network,
        pseudo_network::{PseudoConnector, PseudoHost, PseudoNetwork},
    };
}
