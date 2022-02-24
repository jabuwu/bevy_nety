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
    assert_eq!(app.introspect().test_game_events_on_client.len(), 1);
    assert_eq!(
        app.introspect().test_game_events_on_client[0].data.foo,
        "bar"
    );
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
    assert_eq!(app.introspect().test_game_events_on_client.len(), 0);
}

#[test]
fn local_send_to_players() {
    let mut app = App::new();
    app.setup_for_tests();
    app.network_mut().start_local();
    flush_network(vec![&mut app]);
    let recipients1 = vec![];
    let recipients2 = vec![app.network().me().unwrap()];
    app.network_mut().server_mut().unwrap().send_to_players(
        &recipients1,
        TestGameEvent {
            foo: "no one".into(),
        },
    );
    app.network_mut().server_mut().unwrap().send_to_players(
        &recipients2,
        TestGameEvent {
            foo: "local".into(),
        },
    );
    flush_network(vec![&mut app]);
    assert_eq!(app.introspect().test_game_events_on_client.len(), 1);
    assert_eq!(
        app.introspect().test_game_events_on_client[0].data.foo,
        "local"
    );
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
    assert_eq!(app.introspect().test_game_events_on_client.len(), 1);
    assert_eq!(
        app.introspect().test_game_events_on_client[0].data.foo,
        "bar"
    );
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
    assert_eq!(app.introspect().test_game_events_on_client.len(), 0);
}

#[test]
fn server_client_send_to_players() {
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
    let recipients1 = vec![server_app.network().me().unwrap()];
    let recipients2 = vec![client_app.network().me().unwrap()];
    server_app
        .network_mut()
        .server_mut()
        .unwrap()
        .send_to_players(
            &recipients1,
            TestGameEvent {
                foo: "server".into(),
            },
        );
    server_app
        .network_mut()
        .server_mut()
        .unwrap()
        .send_to_players(
            &recipients2,
            TestGameEvent {
                foo: "client".into(),
            },
        );
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.introspect().test_game_events_on_client.len(), 1);
    assert_eq!(
        server_app.introspect().test_game_events_on_client[0]
            .data
            .foo,
        "server"
    );
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
    assert_eq!(app.introspect().test_game_events_on_client.len(), 0);
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
    assert_eq!(app.introspect().test_game_events_on_client.len(), 0);
}

#[test]
fn server_send_to_players() {
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
    let recipients1 = vec![];
    let recipients2 = vec![client_app.network().me().unwrap()];
    server_app
        .network_mut()
        .server_mut()
        .unwrap()
        .send_to_players(&recipients1, TestGameEvent { foo: "bar".into() });
    server_app
        .network_mut()
        .server_mut()
        .unwrap()
        .send_to_players(&recipients2, TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.introspect().test_game_events_on_client.len(), 0);
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
    assert_eq!(client1_app.introspect().test_game_events_on_client.len(), 1);
    assert_eq!(
        client1_app.introspect().test_game_events_on_client[0]
            .data
            .foo,
        "bar"
    );
    assert_eq!(client2_app.introspect().test_game_events_on_client.len(), 1);
    assert_eq!(
        client2_app.introspect().test_game_events_on_client[0]
            .data
            .foo,
        "bar"
    );
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
    assert_eq!(client1_app.introspect().test_game_events_on_client.len(), 1);
    assert_eq!(
        client1_app.introspect().test_game_events_on_client[0]
            .data
            .foo,
        "bar"
    );
    assert_eq!(client2_app.introspect().test_game_events_on_client.len(), 1);
    assert_eq!(
        client2_app.introspect().test_game_events_on_client[0]
            .data
            .foo,
        "bar"
    );
}

#[test]
fn client_send_to_players() {
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
    let recipients1 = vec![];
    let recipients2 = vec![client1_app.network().me().unwrap()];
    let recipients3 = vec![client2_app.network().me().unwrap()];
    server_app
        .network_mut()
        .server_mut()
        .unwrap()
        .send_to_players(
            &recipients1,
            TestGameEvent {
                foo: "no one".into(),
            },
        );
    server_app
        .network_mut()
        .server_mut()
        .unwrap()
        .send_to_players(
            &recipients2,
            TestGameEvent {
                foo: "client1".into(),
            },
        );
    server_app
        .network_mut()
        .server_mut()
        .unwrap()
        .send_to_players(
            &recipients3,
            TestGameEvent {
                foo: "client2".into(),
            },
        );
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(client1_app.introspect().test_game_events_on_client.len(), 1);
    assert_eq!(
        client1_app.introspect().test_game_events_on_client[0]
            .data
            .foo,
        "client1"
    );
    assert_eq!(client2_app.introspect().test_game_events_on_client.len(), 1);
    assert_eq!(
        client2_app.introspect().test_game_events_on_client[0]
            .data
            .foo,
        "client2"
    );
}
