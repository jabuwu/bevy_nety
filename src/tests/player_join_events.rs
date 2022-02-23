use super::common::prelude::*;
use bevy::prelude::*;

#[test]
fn local_single() {
    let mut app = App::new();
    app.setup_for_tests();
    app.network_mut().start_local();
    flush_network(vec![&mut app]);
    assert_eq!(app.introspect().player_join_events.len(), 1);
    assert_eq!(app.introspect().player_join_events[0].me, true);
    assert_eq!(
        app.introspect().player_join_events[0].existing_player,
        false
    );
    assert_eq!(
        app.introspect().player_join_events[0].player,
        app.network().me().unwrap()
    );
}

#[test]
fn server_client_single() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut app = App::new();
    app.setup_for_tests();
    app.network_mut()
        .start_server_client(vec![pseudo_net.create_host()]);
    flush_network(vec![&mut app]);
    assert_eq!(app.introspect().player_join_events.len(), 1);
    assert_eq!(app.introspect().player_join_events[0].me, true);
    assert_eq!(
        app.introspect().player_join_events[0].existing_player,
        false
    );
    assert_eq!(
        app.introspect().player_join_events[0].player,
        app.network().me().unwrap()
    );
}

#[test]
fn server_client_multiple() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client_app = App::new();
    server_app.setup_for_tests();
    client_app.setup_for_tests();
    server_app
        .network_mut()
        .start_server_client(vec![pseudo_net.create_host()]);
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.introspect().player_join_events.len(), 1);
    assert_eq!(server_app.introspect().player_join_events[0].me, true);
    assert_eq!(
        server_app.introspect().player_join_events[0].existing_player,
        false
    );
    assert_eq!(
        server_app.introspect().player_join_events[0].player,
        server_app.network().me().unwrap()
    );
    client_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.introspect().player_join_events.len(), 2);
    assert_eq!(server_app.introspect().player_join_events[1].me, false);
    assert_eq!(
        server_app.introspect().player_join_events[1].existing_player,
        false
    );
}

#[test]
fn server_single() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client_app = App::new();
    server_app.setup_for_tests();
    client_app.setup_for_tests();
    server_app
        .network_mut()
        .start_server(vec![pseudo_net.create_host()]);
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.introspect().player_join_events.len(), 0);
    client_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.introspect().player_join_events.len(), 1);
    assert_eq!(server_app.introspect().player_join_events[0].me, false);
    assert_eq!(
        server_app.introspect().player_join_events[0].existing_player,
        false
    );
}

#[test]
fn server_multiple() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client1_app = App::new();
    let mut client2_app = App::new();
    server_app.setup_for_tests();
    client1_app.setup_for_tests();
    client2_app.setup_for_tests();
    server_app
        .network_mut()
        .start_server(vec![pseudo_net.create_host()]);
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.introspect().player_join_events.len(), 0);
    client1_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.introspect().player_join_events.len(), 1);
    assert_eq!(server_app.introspect().player_join_events[0].me, false);
    assert_eq!(
        server_app.introspect().player_join_events[0].existing_player,
        false
    );
    client2_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.introspect().player_join_events.len(), 2);
    assert_eq!(server_app.introspect().player_join_events[1].me, false);
    assert_eq!(
        server_app.introspect().player_join_events[1].existing_player,
        false
    );
}

#[test]
fn client_single() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client_app = App::new();
    server_app.setup_for_tests();
    client_app.setup_for_tests();
    server_app
        .network_mut()
        .start_server(vec![pseudo_net.create_host()]);
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(client_app.introspect().player_join_events.len(), 0);
    client_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(client_app.introspect().player_join_events.len(), 1);
    assert_eq!(client_app.introspect().player_join_events[0].me, true);
    assert_eq!(
        client_app.introspect().player_join_events[0].existing_player,
        false
    );
    assert_eq!(
        client_app.introspect().player_join_events[0].player,
        client_app.network().me().unwrap()
    );
}

#[test]
fn client_multiple() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client1_app = App::new();
    let mut client2_app = App::new();
    server_app.setup_for_tests();
    client1_app.setup_for_tests();
    client2_app.setup_for_tests();
    server_app
        .network_mut()
        .start_server(vec![pseudo_net.create_host()]);
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(client1_app.introspect().player_join_events.len(), 0);
    client1_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(client1_app.introspect().player_join_events.len(), 1);
    assert_eq!(client1_app.introspect().player_join_events[0].me, true);
    assert_eq!(
        client1_app.introspect().player_join_events[0].existing_player,
        false
    );
    assert_eq!(
        client1_app.introspect().player_join_events[0].player,
        client1_app.network().me().unwrap()
    );
    assert_eq!(client2_app.introspect().player_join_events.len(), 0);
    client2_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(client1_app.introspect().player_join_events.len(), 2);
    assert_eq!(client1_app.introspect().player_join_events[1].me, false);
    assert_eq!(
        client1_app.introspect().player_join_events[1].existing_player,
        false
    );
    assert_eq!(
        client1_app.introspect().player_join_events[1].player,
        client2_app.network().me().unwrap()
    );
    assert_eq!(client2_app.introspect().player_join_events.len(), 2);
    assert_eq!(client2_app.introspect().player_join_events[0].me, false);
    assert_eq!(
        client2_app.introspect().player_join_events[0].existing_player,
        true
    );
    assert_eq!(
        client2_app.introspect().player_join_events[0].player,
        client1_app.network().me().unwrap()
    );
    assert_eq!(client2_app.introspect().player_join_events[1].me, true);
    assert_eq!(
        client2_app.introspect().player_join_events[1].existing_player,
        false
    );
    assert_eq!(
        client2_app.introspect().player_join_events[1].player,
        client2_app.network().me().unwrap()
    );
}
