use super::common::prelude::*;
use bevy::prelude::*;

#[test]
pub fn local() {
    let mut app = App::new();
    app.setup_for_tests();
    assert_eq!(app.network_mut().is_server(), false);
    assert_eq!(app.network_mut().is_client(), false);
    assert_eq!(app.network_mut().is_connecting(), false);
    assert_eq!(app.network_mut().is_connected(), false);
    assert_eq!(app.network_mut().is_disconnected(), true);
    app.network_mut().start_local();
    assert_eq!(app.network_mut().is_server(), true);
    assert_eq!(app.network_mut().is_client(), true);
    assert_eq!(app.network_mut().is_connecting(), false);
    assert_eq!(app.network_mut().is_connected(), true);
    assert_eq!(app.network_mut().is_disconnected(), false);
    app.network_mut().stop();
    assert_eq!(app.network_mut().is_server(), false);
    assert_eq!(app.network_mut().is_client(), false);
    assert_eq!(app.network_mut().is_connecting(), false);
    assert_eq!(app.network_mut().is_connected(), false);
    assert_eq!(app.network_mut().is_disconnected(), true);
}

#[test]
pub fn server_client() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut app = App::new();
    app.setup_for_tests();
    assert_eq!(app.network_mut().is_server(), false);
    assert_eq!(app.network_mut().is_client(), false);
    assert_eq!(app.network_mut().is_connecting(), false);
    assert_eq!(app.network_mut().is_connected(), false);
    assert_eq!(app.network_mut().is_disconnected(), true);
    app.network_mut()
        .start_server_client(vec![pseudo_net.create_host()]);
    assert_eq!(app.network_mut().is_server(), true);
    assert_eq!(app.network_mut().is_client(), true);
    assert_eq!(app.network_mut().is_connecting(), false);
    assert_eq!(app.network_mut().is_connected(), true);
    assert_eq!(app.network_mut().is_disconnected(), false);
    app.network_mut().stop();
    assert_eq!(app.network_mut().is_server(), false);
    assert_eq!(app.network_mut().is_client(), false);
    assert_eq!(app.network_mut().is_connecting(), false);
    assert_eq!(app.network_mut().is_connected(), false);
    assert_eq!(app.network_mut().is_disconnected(), true);
}

#[test]
pub fn server() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut app = App::new();
    app.setup_for_tests();
    assert_eq!(app.network_mut().is_server(), false);
    assert_eq!(app.network_mut().is_client(), false);
    assert_eq!(app.network_mut().is_connecting(), false);
    assert_eq!(app.network_mut().is_connected(), false);
    assert_eq!(app.network_mut().is_disconnected(), true);
    app.network_mut()
        .start_server(vec![pseudo_net.create_host()]);
    assert_eq!(app.network_mut().is_server(), true);
    assert_eq!(app.network_mut().is_client(), false);
    assert_eq!(app.network_mut().is_connecting(), false);
    assert_eq!(app.network_mut().is_connected(), true);
    assert_eq!(app.network_mut().is_disconnected(), false);
    app.network_mut().stop();
    assert_eq!(app.network_mut().is_server(), false);
    assert_eq!(app.network_mut().is_client(), false);
    assert_eq!(app.network_mut().is_connecting(), false);
    assert_eq!(app.network_mut().is_connected(), false);
    assert_eq!(app.network_mut().is_disconnected(), true);
}

#[test]
pub fn client() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client_app = App::new();
    server_app.setup_for_tests();
    client_app.setup_for_tests();
    assert_eq!(client_app.network_mut().is_server(), false);
    assert_eq!(client_app.network_mut().is_client(), false);
    assert_eq!(client_app.network_mut().is_connecting(), false);
    assert_eq!(client_app.network_mut().is_connected(), false);
    assert_eq!(client_app.network_mut().is_disconnected(), true);
    server_app
        .network_mut()
        .start_server(vec![pseudo_net.create_host()]);
    client_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    assert_eq!(client_app.network_mut().is_server(), false);
    assert_eq!(client_app.network_mut().is_client(), false);
    assert_eq!(client_app.network_mut().is_connecting(), true);
    assert_eq!(client_app.network_mut().is_connected(), false);
    assert_eq!(client_app.network_mut().is_disconnected(), false);
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(client_app.network_mut().is_server(), false);
    assert_eq!(client_app.network_mut().is_client(), true);
    assert_eq!(client_app.network_mut().is_connecting(), false);
    assert_eq!(client_app.network_mut().is_connected(), true);
    assert_eq!(client_app.network_mut().is_disconnected(), false);
    client_app.network_mut().stop();
    assert_eq!(client_app.network_mut().is_server(), false);
    assert_eq!(client_app.network_mut().is_client(), false);
    assert_eq!(client_app.network_mut().is_connecting(), false);
    assert_eq!(client_app.network_mut().is_connected(), false);
    assert_eq!(client_app.network_mut().is_disconnected(), true);
}
