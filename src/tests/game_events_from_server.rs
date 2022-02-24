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
fn client_send_to_all() {
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
    server_app
        .network_mut()
        .server_mut()
        .unwrap()
        .send_to_all(TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(client_app.introspect().test_game_events.len(), 1);
    assert_eq!(client_app.introspect().test_game_events[0].foo, "bar");
}
