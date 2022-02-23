use super::common::prelude::*;
use bevy::prelude::*;

#[test]
pub fn local() {
    let mut app = App::new();
    app.setup_for_tests();
    app.network_mut().start_local();
    flush_network(vec![&mut app]);
    assert_eq!(app.introspect().connect_events.len(), 1);
    assert_eq!(app.introspect().connect_events[0].is_server, true);
    assert_eq!(app.introspect().connect_events[0].is_client, true);
    assert_eq!(app.introspect().connecting_events.len(), 0);
    assert_eq!(app.introspect().disconnect_events.len(), 0);
    app.network_mut().stop();
    flush_network(vec![&mut app]);
    assert_eq!(app.introspect().connect_events.len(), 1);
    assert_eq!(app.introspect().connecting_events.len(), 0);
    assert_eq!(app.introspect().disconnect_events.len(), 1);
    assert_eq!(
        app.introspect().disconnect_events[0].failed_to_connect,
        false
    );
}

#[test]
pub fn server_client() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client_app = App::new();
    server_app.setup_for_tests();
    client_app.setup_for_tests();
    server_app
        .network_mut()
        .start_server_client(vec![pseudo_net.create_host()]);
    client_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.introspect().connect_events.len(), 1);
    assert_eq!(server_app.introspect().connect_events[0].is_server, true);
    assert_eq!(server_app.introspect().connect_events[0].is_client, true);
    assert_eq!(server_app.introspect().connecting_events.len(), 0);
    assert_eq!(server_app.introspect().disconnect_events.len(), 0);
    server_app.network_mut().stop();
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.introspect().connect_events.len(), 1);
    assert_eq!(server_app.introspect().connecting_events.len(), 0);
    assert_eq!(server_app.introspect().disconnect_events.len(), 1);
    assert_eq!(
        server_app.introspect().disconnect_events[0].failed_to_connect,
        false
    );
}

#[test]
pub fn server() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client_app = App::new();
    server_app.setup_for_tests();
    client_app.setup_for_tests();
    server_app
        .network_mut()
        .start_server(vec![pseudo_net.create_host()]);
    client_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.introspect().connect_events.len(), 1);
    assert_eq!(server_app.introspect().connect_events[0].is_server, true);
    assert_eq!(server_app.introspect().connect_events[0].is_client, false);
    assert_eq!(server_app.introspect().connecting_events.len(), 0);
    assert_eq!(server_app.introspect().disconnect_events.len(), 0);
    server_app.network_mut().stop();
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.introspect().connect_events.len(), 1);
    assert_eq!(server_app.introspect().connecting_events.len(), 0);
    assert_eq!(server_app.introspect().disconnect_events.len(), 1);
    assert_eq!(
        server_app.introspect().disconnect_events[0].failed_to_connect,
        false
    );
}

#[test]
pub fn client_stop() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client_app = App::new();
    server_app.setup_for_tests();
    client_app.setup_for_tests();
    server_app
        .network_mut()
        .start_server(vec![pseudo_net.create_host()]);
    let (connector, acceptor) = pseudo_net.create_connector().as_pending();
    client_app.network_mut().start_client(connector);
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(client_app.introspect().connect_events.len(), 0);
    assert_eq!(client_app.introspect().connecting_events.len(), 1);
    assert_eq!(client_app.introspect().disconnect_events.len(), 0);
    acceptor.success();
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(client_app.introspect().connect_events.len(), 1);
    assert_eq!(client_app.introspect().connect_events[0].is_server, false);
    assert_eq!(client_app.introspect().connect_events[0].is_client, true);
    assert_eq!(client_app.introspect().connecting_events.len(), 1);
    client_app.network_mut().stop();
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(client_app.introspect().disconnect_events.len(), 1);
    assert_eq!(
        client_app.introspect().disconnect_events[0].failed_to_connect,
        false
    );
}

#[test]
pub fn client_disconnect() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client_app = App::new();
    server_app.setup_for_tests();
    client_app.setup_for_tests();
    server_app
        .network_mut()
        .start_server(vec![pseudo_net.create_host()]);
    let (connector, acceptor) = pseudo_net.create_connector().as_pending();
    client_app.network_mut().start_client(connector);
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(client_app.introspect().connect_events.len(), 0);
    assert_eq!(client_app.introspect().connecting_events.len(), 1);
    assert_eq!(client_app.introspect().disconnect_events.len(), 0);
    acceptor.success();
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(client_app.introspect().connect_events.len(), 1);
    assert_eq!(client_app.introspect().connect_events[0].is_server, false);
    assert_eq!(client_app.introspect().connect_events[0].is_client, true);
    assert_eq!(client_app.introspect().connecting_events.len(), 1);
    server_app.network_mut().stop();
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(client_app.introspect().disconnect_events.len(), 1);
    assert_eq!(
        client_app.introspect().disconnect_events[0].failed_to_connect,
        false
    );
}

#[test]
pub fn client_failed_to_connect() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client_app = App::new();
    server_app.setup_for_tests();
    client_app.setup_for_tests();
    server_app
        .network_mut()
        .start_server(vec![pseudo_net.create_host()]);
    let (connector, acceptor) = pseudo_net.create_connector().as_pending();
    client_app.network_mut().start_client(connector);
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(client_app.introspect().connect_events.len(), 0);
    assert_eq!(client_app.introspect().connecting_events.len(), 1);
    assert_eq!(client_app.introspect().disconnect_events.len(), 0);
    acceptor.fail();
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(client_app.introspect().connect_events.len(), 0);
    assert_eq!(client_app.introspect().connecting_events.len(), 1);
    assert_eq!(client_app.introspect().disconnect_events.len(), 1);
    assert_eq!(
        client_app.introspect().disconnect_events[0].failed_to_connect,
        true
    );
}
