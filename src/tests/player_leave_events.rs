use super::common::prelude::*;
use bevy::prelude::*;

#[test]
fn local_none() {
    let mut app = App::new();
    app.setup_for_tests();
    app.network_mut().start_local();
    flush_network(vec![&mut app]);
    app.network_mut().stop();
    flush_network(vec![&mut app]);
    assert_eq!(app.introspect().player_leave_events.len(), 0);
}

#[test]
fn server_client_none() {
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
    server_app.network_mut().stop();
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.introspect().player_leave_events.len(), 0);
}

#[test]
fn server_client_one() {
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
    let client_me = client_app.network().me().unwrap();
    client_app.network_mut().stop();
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.introspect().player_leave_events.len(), 1);
    assert_eq!(
        server_app.introspect().player_leave_events[0].player,
        client_me
    );
}

#[test]
fn server_client_multiple() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client1_app = App::new();
    let mut client2_app = App::new();
    server_app.setup_for_tests();
    client1_app.setup_for_tests();
    client2_app.setup_for_tests();
    server_app
        .network_mut()
        .start_server_client(vec![pseudo_net.create_host()]);
    client1_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    client2_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    let client1_me = client1_app.network().me().unwrap();
    let client2_me = client2_app.network().me().unwrap();
    client1_app.network_mut().stop();
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.introspect().player_leave_events.len(), 1);
    assert_eq!(
        server_app.introspect().player_leave_events[0].player,
        client1_me
    );
    client2_app.network_mut().stop();
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.introspect().player_leave_events.len(), 2);
    assert_eq!(
        server_app.introspect().player_leave_events[1].player,
        client2_me
    );
}

#[test]
fn server_none() {
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
    server_app.network_mut().stop();
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.introspect().player_leave_events.len(), 0);
}

#[test]
fn server_one() {
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
    let client_me = client_app.network().me().unwrap();
    client_app.network_mut().stop();
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.introspect().player_leave_events.len(), 1);
    assert_eq!(
        server_app.introspect().player_leave_events[0].player,
        client_me
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
    client1_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    client2_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    let client1_me = client1_app.network().me().unwrap();
    let client2_me = client2_app.network().me().unwrap();
    client1_app.network_mut().stop();
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.introspect().player_leave_events.len(), 1);
    assert_eq!(
        server_app.introspect().player_leave_events[0].player,
        client1_me
    );
    client2_app.network_mut().stop();
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.introspect().player_leave_events.len(), 2);
    assert_eq!(
        server_app.introspect().player_leave_events[1].player,
        client2_me
    );
}

#[test]
fn client_none() {
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
    server_app.network_mut().stop();
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(client_app.introspect().player_leave_events.len(), 0);
}

#[test]
fn client_one() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client1_app = App::new();
    let mut client2_app = App::new();
    server_app.setup_for_tests();
    client1_app.setup_for_tests();
    client2_app.setup_for_tests();
    server_app
        .network_mut()
        .start_server_client(vec![pseudo_net.create_host()]);
    client1_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    client2_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    let client1_me = client1_app.network().me().unwrap();
    client1_app.network_mut().stop();
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(client2_app.introspect().player_leave_events.len(), 1);
    assert_eq!(
        client2_app.introspect().player_leave_events[0].player,
        client1_me
    );
}

#[test]
fn client_multiple() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client1_app = App::new();
    let mut client2_app = App::new();
    let mut client3_app = App::new();
    server_app.setup_for_tests();
    client1_app.setup_for_tests();
    client2_app.setup_for_tests();
    client3_app.setup_for_tests();
    server_app
        .network_mut()
        .start_server_client(vec![pseudo_net.create_host()]);
    client1_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    client2_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    client3_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    flush_network(vec![
        &mut server_app,
        &mut client1_app,
        &mut client2_app,
        &mut client3_app,
    ]);
    let client1_me = client1_app.network().me().unwrap();
    let client2_me = client2_app.network().me().unwrap();
    client1_app.network_mut().stop();
    flush_network(vec![
        &mut server_app,
        &mut client1_app,
        &mut client2_app,
        &mut client3_app,
    ]);
    assert_eq!(client2_app.introspect().player_leave_events.len(), 1);
    assert_eq!(
        client2_app.introspect().player_leave_events[0].player,
        client1_me
    );
    assert_eq!(client3_app.introspect().player_leave_events.len(), 1);
    assert_eq!(
        client3_app.introspect().player_leave_events[0].player,
        client1_me
    );
    client2_app.network_mut().stop();
    flush_network(vec![
        &mut server_app,
        &mut client1_app,
        &mut client2_app,
        &mut client3_app,
    ]);
    assert_eq!(client3_app.introspect().player_leave_events.len(), 2);
    assert_eq!(
        client3_app.introspect().player_leave_events[1].player,
        client2_me
    );
}
