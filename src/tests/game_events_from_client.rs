use super::common::prelude::*;

// Test the functionality of client.send()
// The server should always receive this event, even if we are the server

#[test]
fn local_send() {
    // Test that local server receives event from itself

    let mut env = TestEnvironment::default();

    env.create_local("local");
    env.flush_network();
    env["local"]
        .client()
        .send(TestGameEvent { foo: "bar".into() });
    env.flush_network();

    let me = env["local"].network().me().unwrap();
    assert_eq!(
        env["local"].introspect().test_game_events_on_server.len(),
        1
    );
    assert_eq!(
        env["local"].introspect().test_game_events_on_server[0].from,
        me,
    );
    assert_eq!(
        env["local"].introspect().test_game_events_on_server[0]
            .data
            .foo,
        "bar"
    );
}

#[test]
fn server_client_send() {
    // Test that server-client receives event from itself

    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.flush_network();
    env["server"]
        .client()
        .send(TestGameEvent { foo: "bar".into() });
    env.flush_network();

    let server_me = env["server"].network().me().unwrap();
    assert_eq!(
        env["server"].introspect().test_game_events_on_server.len(),
        1
    );
    assert_eq!(
        env["server"].introspect().test_game_events_on_server[0].from,
        server_me
    );
    assert_eq!(
        env["server"].introspect().test_game_events_on_server[0]
            .data
            .foo,
        "bar"
    );
}

#[test]
fn client_send() {
    // Test that server receives event from client

    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.create_client("client", "server");
    env.flush_network();
    env["client"]
        .client()
        .send(TestGameEvent { foo: "bar".into() });
    env.flush_network();

    let client_me = env["client"].network().me().unwrap();
    assert_eq!(
        env["server"].introspect().test_game_events_on_server.len(),
        1
    );
    assert_eq!(
        env["server"].introspect().test_game_events_on_server[0].from,
        client_me
    );
    assert_eq!(
        env["server"].introspect().test_game_events_on_server[0]
            .data
            .foo,
        "bar"
    );
}
