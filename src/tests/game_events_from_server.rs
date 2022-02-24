use super::common::prelude::*;
use bevy::prelude::*;

#[test]
fn local_send_to_all() {
    let mut app = App::new();
    app.setup_for_tests();
    app.network_mut().start_local();
    flush_network(vec![&mut app]);
    app.network_mut()
        .server_mut()
        .unwrap()
        .send_to_all(TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut app]);
    assert_eq!(app.introspect().test_game_events.len(), 1);
    assert_eq!(app.introspect().test_game_events[0].foo, "bar");
}

#[test]
fn local_send_to_all_except_local() {
    let mut app = App::new();
    app.setup_for_tests();
    app.network_mut().start_local();
    flush_network(vec![&mut app]);
    app.network_mut()
        .server_mut()
        .unwrap()
        .send_to_all_except_local(TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut app]);
    assert_eq!(app.introspect().test_game_events.len(), 0);
}

#[test]
fn server_client_send_to_all() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut app = App::new();
    app.setup_for_tests();
    app.network_mut()
        .start_server_client(vec![pseudo_net.create_host()]);
    flush_network(vec![&mut app]);
    app.network_mut()
        .server_mut()
        .unwrap()
        .send_to_all(TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut app]);
    assert_eq!(app.introspect().test_game_events.len(), 1);
    assert_eq!(app.introspect().test_game_events[0].foo, "bar");
}

#[test]
fn server_client_send_to_all_except_local() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut app = App::new();
    app.setup_for_tests();
    app.network_mut()
        .start_server_client(vec![pseudo_net.create_host()]);
    flush_network(vec![&mut app]);
    app.network_mut()
        .server_mut()
        .unwrap()
        .send_to_all_except_local(TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut app]);
    assert_eq!(app.introspect().test_game_events.len(), 0);
}

#[test]
fn server_send_to_all() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut app = App::new();
    app.setup_for_tests();
    app.network_mut()
        .start_server(vec![pseudo_net.create_host()]);
    flush_network(vec![&mut app]);
    app.network_mut()
        .server_mut()
        .unwrap()
        .send_to_all(TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut app]);
    assert_eq!(app.introspect().test_game_events.len(), 0);
}

#[test]
fn server_send_to_all_except_local() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut app = App::new();
    app.setup_for_tests();
    app.network_mut()
        .start_server(vec![pseudo_net.create_host()]);
    flush_network(vec![&mut app]);
    app.network_mut()
        .server_mut()
        .unwrap()
        .send_to_all(TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut app]);
    assert_eq!(app.introspect().test_game_events.len(), 0);
}

#[test]
fn client_send_to_all() {
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
    server_app
        .network_mut()
        .server_mut()
        .unwrap()
        .send_to_all(TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(client1_app.introspect().test_game_events.len(), 1);
    assert_eq!(client1_app.introspect().test_game_events[0].foo, "bar");
    assert_eq!(client2_app.introspect().test_game_events.len(), 1);
    assert_eq!(client2_app.introspect().test_game_events[0].foo, "bar");
}

#[test]
fn client_send_to_all_except_local() {
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
    server_app
        .network_mut()
        .server_mut()
        .unwrap()
        .send_to_all_except_local(TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(client1_app.introspect().test_game_events.len(), 1);
    assert_eq!(client1_app.introspect().test_game_events[0].foo, "bar");
    assert_eq!(client2_app.introspect().test_game_events.len(), 1);
    assert_eq!(client2_app.introspect().test_game_events[0].foo, "bar");
}
