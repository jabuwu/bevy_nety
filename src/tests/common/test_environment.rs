use super::{
    app_setup_for_tests::AppSetupForTests,
    introspection::Introspection,
    pseudo_network::{PseudoConnectorAccepter, PseudoNetwork},
};
use crate::prelude::*;
use bevy::prelude::*;
use core::ops::{Index, IndexMut};
use std::collections::HashMap;

#[derive(Default)]
pub struct TestEnvironment {
    pseudo_network: PseudoNetwork,
    test_apps: HashMap<String, TestApp>,
}

impl TestEnvironment {
    pub fn create_app(&mut self, name: &str) {
        let mut test_app = TestApp::new();
        test_app.app.setup_for_tests();
        self.test_apps.insert(name.into(), test_app);
    }

    pub fn create_local(&mut self, name: &str) {
        self.create_app(name);
        self.start_local(name);
    }

    pub fn create_server(&mut self, name: &str) {
        self.create_app(name);
        self.start_server(name);
    }

    pub fn create_server_client(&mut self, name: &str) {
        self.create_app(name);
        self.start_server_client(name);
    }

    pub fn create_client(&mut self, name: &str, server: &str) {
        self.create_app(name);
        self.start_client(name, server);
    }

    pub fn create_client_pending(&mut self, name: &str, server: &str) -> PseudoConnectorAccepter {
        self.create_app(name);
        self.start_client_pending(name, server)
    }

    pub fn start_local(&mut self, name: &str) {
        self[name].network().start_local();
    }

    pub fn start_server(&mut self, name: &str) {
        let host = self.pseudo_network.create_host_named(name);
        self[name].network().start_server(vec![host]);
    }

    pub fn start_server_client(&mut self, name: &str) {
        let host = self.pseudo_network.create_host_named(name);
        self[name].network().start_server_client(vec![host]);
    }

    pub fn start_client(&mut self, name: &str, server: &str) {
        let connector = self.pseudo_network.create_connector_named(server);
        self[name].network().start_client(connector.as_success());
    }

    pub fn start_client_pending(&mut self, name: &str, server: &str) -> PseudoConnectorAccepter {
        let connector = self.pseudo_network.create_connector_named(server);
        let (connector, acceptor) = connector.as_pending();
        self[name].network().start_client(connector);
        acceptor
    }

    pub fn flush_network(&mut self) {
        for _ in 0..10 {
            for (_, test_app) in self.test_apps.iter_mut() {
                test_app.app().update();
            }
        }
    }
}

impl Index<&str> for TestEnvironment {
    type Output = TestApp;
    fn index(&self, index: &str) -> &Self::Output {
        self.test_apps.get(index).unwrap()
    }
}

impl IndexMut<&str> for TestEnvironment {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.test_apps.get_mut(index).unwrap()
    }
}

pub struct TestApp {
    app: App,
}

impl TestApp {
    pub fn new() -> Self {
        Self { app: App::new() }
    }

    pub fn app(&mut self) -> &mut App {
        &mut self.app
    }

    pub fn world(&mut self) -> &mut World {
        &mut self.app.world
    }

    pub fn network(&mut self) -> Mut<Network> {
        self.app.world.get_resource_mut::<Network>().unwrap()
    }

    pub fn server(&mut self) -> &mut NetworkServer {
        let network = unsafe { &mut *(self.network().as_mut() as *mut Network) };
        network.server_mut().unwrap()
    }

    pub fn client(&mut self) -> &mut NetworkClient {
        let network = unsafe { &mut *(self.network().as_mut() as *mut Network) };
        network.client_mut().unwrap()
    }

    pub fn introspect(&mut self) -> Mut<Introspection> {
        self.app.world.get_resource_mut::<Introspection>().unwrap()
    }
}
