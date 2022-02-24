use super::common::prelude::*;
use bevy::prelude::*;

#[test]
fn local_send() {
    let mut app = App::new();
    app.setup_for_tests();
    app.network_mut().start_local();
    flush_network(vec![&mut app]);
    app.network_mut()
        .client_mut()
        .unwrap()
        .send(TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut app]);
    assert_eq!(app.introspect().test_game_events_on_server.len(), 1);
    assert_eq!(
        app.introspect().test_game_events_on_server[0].from,
        app.network().me().unwrap()
    );
    assert_eq!(
        app.introspect().test_game_events_on_server[0].data.foo,
        "bar"
    );
}

#[test]
fn server_client_send() {
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
    client_app
        .network_mut()
        .client_mut()
        .unwrap()
        .send(TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.introspect().test_game_events_on_server.len(), 1);
    assert_eq!(
        server_app.introspect().test_game_events_on_server[0].from,
        client_app.network().me().unwrap()
    );
    assert_eq!(
        server_app.introspect().test_game_events_on_server[0]
            .data
            .foo,
        "bar"
    );
}

#[test]
fn server_send() {
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
    client_app
        .network_mut()
        .client_mut()
        .unwrap()
        .send(TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.introspect().test_game_events_on_server.len(), 1);
    assert_eq!(
        server_app.introspect().test_game_events_on_server[0].from,
        client_app.network().me().unwrap()
    );
    assert_eq!(
        server_app.introspect().test_game_events_on_server[0]
            .data
            .foo,
        "bar"
    );
}

#[test]
fn client_send() {
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
    client_app
        .network_mut()
        .client_mut()
        .unwrap()
        .send(TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(client_app.introspect().test_game_events_on_server.len(), 0);
}
