use super::common::prelude::*;

// Test the functionality of:
// - server.send_to_all()
// - server.send_to_all_except_local()
// - server.send_to_players()

#[test]
fn local_send_to_all() {
    // Test server.send_to_all() for local server

    let mut env = TestEnvironment::default();

    env.create_local("local");
    env.flush_network();
    env["local"]
        .server()
        .send_to_all(TestGameEvent { foo: "bar".into() });
    env.flush_network();

    assert_eq!(
        env["local"].introspect().test_game_events_on_client.len(),
        1
    );
    assert_eq!(
        env["local"].introspect().test_game_events_on_client[0]
            .data
            .foo,
        "bar"
    );
}

#[test]
fn local_send_to_all_except_local() {
    // Test server.send_to_all_except_local() for local server

    let mut env = TestEnvironment::default();

    env.create_local("local");
    env.flush_network();
    env["local"]
        .server()
        .send_to_all_except_local(TestGameEvent { foo: "bar".into() });
    env.flush_network();

    assert_eq!(
        env["local"].introspect().test_game_events_on_client.len(),
        0
    );
}

#[test]
fn local_send_to_players() {
    // Test server.send_to_all_players() for local server

    let mut env = TestEnvironment::default();

    env.create_local("local");
    env.flush_network();

    let recipients1 = vec![];
    let recipients2 = vec![env["local"].network().me().unwrap()];
    env["local"].server().send_to_players(
        &recipients1,
        TestGameEvent {
            foo: "no one".into(),
        },
    );
    env["local"].server().send_to_players(
        &recipients2,
        TestGameEvent {
            foo: "local".into(),
        },
    );
    env.flush_network();

    assert_eq!(
        env["local"].introspect().test_game_events_on_client.len(),
        1
    );
    assert_eq!(
        env["local"].introspect().test_game_events_on_client[0]
            .data
            .foo,
        "local"
    );
}

#[test]
fn server_client_send_to_all() {
    // Test server.send_to_all() for server-client

    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.flush_network();

    env["server"]
        .server()
        .send_to_all(TestGameEvent { foo: "bar".into() });
    env.flush_network();

    assert_eq!(
        env["server"].introspect().test_game_events_on_client.len(),
        1
    );
    assert_eq!(
        env["server"].introspect().test_game_events_on_client[0]
            .data
            .foo,
        "bar"
    );
}

#[test]
fn server_client_send_to_all_except_local() {
    // Test server.send_to_all_except_local() for server-client

    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.flush_network();

    env["server"]
        .server()
        .send_to_all_except_local(TestGameEvent { foo: "bar".into() });
    env.flush_network();

    assert_eq!(
        env["server"].introspect().test_game_events_on_client.len(),
        0
    );
}

#[test]
fn server_client_send_to_players() {
    // Test server.send_to_players() for server-client

    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.flush_network();

    let recipients1 = vec![];
    let recipients2 = vec![env["server"].network().me().unwrap()];

    env["server"].server().send_to_players(
        &recipients1,
        TestGameEvent {
            foo: "no one".into(),
        },
    );
    env["server"].server().send_to_players(
        &recipients2,
        TestGameEvent {
            foo: "server".into(),
        },
    );
    env.flush_network();

    assert_eq!(
        env["server"].introspect().test_game_events_on_client.len(),
        1
    );
    assert_eq!(
        env["server"].introspect().test_game_events_on_client[0]
            .data
            .foo,
        "server"
    );
}

#[test]
fn server_send_to_all() {
    // Test server.send_to_all() for server

    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.flush_network();

    env["server"]
        .server()
        .send_to_all(TestGameEvent { foo: "bar".into() });
    env.flush_network();

    assert_eq!(
        env["server"].introspect().test_game_events_on_client.len(),
        0
    );
}

#[test]
fn server_send_to_all_except_local() {
    // Test server.send_to_all_except_local() for server

    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.flush_network();

    env["server"]
        .server()
        .send_to_all_except_local(TestGameEvent { foo: "bar".into() });
    env.flush_network();

    assert_eq!(
        env["server"].introspect().test_game_events_on_client.len(),
        0
    );
}

#[test]
fn server_send_to_players() {
    // Test server.send_to_players() for server

    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_client("client", "server");
    env.flush_network();

    let recipients1 = vec![];
    let recipients2 = vec![env["client"].network().me().unwrap()];

    env["server"].server().send_to_players(
        &recipients1,
        TestGameEvent {
            foo: "no one".into(),
        },
    );
    env["server"].server().send_to_players(
        &recipients2,
        TestGameEvent {
            foo: "client".into(),
        },
    );
    env.flush_network();

    assert_eq!(
        env["server"].introspect().test_game_events_on_client.len(),
        0
    );
}

#[test]
fn client_send_to_all() {
    // This tests sending from server and receiving on client using server.send_to_all()

    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    env["server"]
        .server()
        .send_to_all(TestGameEvent { foo: "bar".into() });
    env.flush_network();

    assert_eq!(
        env["client1"].introspect().test_game_events_on_client.len(),
        1
    );
    assert_eq!(
        env["client1"].introspect().test_game_events_on_client[0]
            .data
            .foo,
        "bar"
    );
    assert_eq!(
        env["client2"].introspect().test_game_events_on_client.len(),
        1
    );
    assert_eq!(
        env["client2"].introspect().test_game_events_on_client[0]
            .data
            .foo,
        "bar"
    );
}

#[test]
fn client_send_to_all_except_local() {
    // This tests sending from server and receiving on client using server.send_to_all_except_local()

    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    env["server"]
        .server()
        .send_to_all_except_local(TestGameEvent { foo: "bar".into() });
    env.flush_network();

    assert_eq!(
        env["client1"].introspect().test_game_events_on_client.len(),
        1
    );
    assert_eq!(
        env["client1"].introspect().test_game_events_on_client[0]
            .data
            .foo,
        "bar"
    );
    assert_eq!(
        env["client2"].introspect().test_game_events_on_client.len(),
        1
    );
    assert_eq!(
        env["client2"].introspect().test_game_events_on_client[0]
            .data
            .foo,
        "bar"
    );
}

#[test]
fn client_send_to_players() {
    // This tests sending from server and receiving on client using server.send_to_players()

    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    let recipients1 = vec![];
    let recipients2 = vec![env["client1"].network().me().unwrap()];
    let recipients3 = vec![env["client2"].network().me().unwrap()];

    env["server"].server().send_to_players(
        &recipients1,
        TestGameEvent {
            foo: "no one".into(),
        },
    );
    env["server"].server().send_to_players(
        &recipients2,
        TestGameEvent {
            foo: "client1".into(),
        },
    );
    env["server"].server().send_to_players(
        &recipients3,
        TestGameEvent {
            foo: "client2".into(),
        },
    );
    env.flush_network();

    assert_eq!(
        env["client1"].introspect().test_game_events_on_client.len(),
        1
    );
    assert_eq!(
        env["client1"].introspect().test_game_events_on_client[0]
            .data
            .foo,
        "client1"
    );
    assert_eq!(
        env["client2"].introspect().test_game_events_on_client.len(),
        1
    );
    assert_eq!(
        env["client2"].introspect().test_game_events_on_client[0]
            .data
            .foo,
        "client2"
    );
}
