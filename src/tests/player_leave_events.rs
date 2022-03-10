use super::common::prelude::*;

#[test]
fn local_none() {
    let mut env = TestEnvironment::default();

    env.create_local("local");
    env.flush_network();
    env["local"].network().stop();
    env.flush_network();
    assert_eq!(env["local"].introspect().player_leave_events.len(), 0);
}

#[test]
fn server_client_none() {
    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.create_client("client", "server");
    env.flush_network();
    env["server"].network().stop();
    env.flush_network();
    assert_eq!(env["server"].introspect().player_leave_events.len(), 0);
}

#[test]
fn server_client_one() {
    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.create_client("client", "server");
    env.flush_network();

    let client_me = env["client"].network().me().unwrap();
    env["client"].network().stop();
    env.flush_network();
    assert_eq!(env["server"].introspect().player_leave_events.len(), 1);
    assert_eq!(
        env["server"].introspect().player_leave_events[0].player,
        client_me
    );
}

#[test]
fn server_client_multiple() {
    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    let client1_me = env["client1"].network().me().unwrap();
    env["client1"].network().stop();
    env.flush_network();
    assert_eq!(env["server"].introspect().player_leave_events.len(), 1);
    assert_eq!(
        env["server"].introspect().player_leave_events[0].player,
        client1_me
    );

    let client2_me = env["client2"].network().me().unwrap();
    env["client2"].network().stop();
    env.flush_network();
    assert_eq!(env["server"].introspect().player_leave_events.len(), 2);
    assert_eq!(
        env["server"].introspect().player_leave_events[1].player,
        client2_me
    );
}

#[test]
fn server_none() {
    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_client("client", "server");
    env.flush_network();
    env["server"].network().stop();
    env.flush_network();
    assert_eq!(env["server"].introspect().player_leave_events.len(), 0);
}

#[test]
fn server_one() {
    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_client("client", "server");
    env.flush_network();

    let client_me = env["client"].network().me().unwrap();
    env["client"].network().stop();
    env.flush_network();

    assert_eq!(env["server"].introspect().player_leave_events.len(), 1);
    assert_eq!(
        env["server"].introspect().player_leave_events[0].player,
        client_me
    );
}

#[test]
fn server_multiple() {
    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    let client1_me = env["client1"].network().me().unwrap();
    env["client1"].network().stop();
    env.flush_network();

    assert_eq!(env["server"].introspect().player_leave_events.len(), 1);
    assert_eq!(
        env["server"].introspect().player_leave_events[0].player,
        client1_me
    );

    let client2_me = env["client2"].network().me().unwrap();
    env["client2"].network().stop();
    env.flush_network();

    assert_eq!(env["server"].introspect().player_leave_events.len(), 2);
    assert_eq!(
        env["server"].introspect().player_leave_events[1].player,
        client2_me
    );
}

#[test]
fn client_none() {
    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_client("client", "server");
    env.flush_network();

    env["server"].network().stop();
    env.flush_network();

    assert_eq!(env["client"].introspect().player_leave_events.len(), 0);
}

#[test]
fn client_one() {
    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    let client1_me = env["client1"].network().me().unwrap();
    env["client1"].network().stop();
    env.flush_network();

    assert_eq!(env["client2"].introspect().player_leave_events.len(), 1);
    assert_eq!(
        env["client2"].introspect().player_leave_events[0].player,
        client1_me
    );
}

#[test]
fn client_multiple() {
    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.create_client("client3", "server");
    env.flush_network();

    let client1_me = env["client1"].network().me().unwrap();
    let client2_me = env["client2"].network().me().unwrap();
    env["client1"].network().stop();
    env.flush_network();

    assert_eq!(env["client2"].introspect().player_leave_events.len(), 1);
    assert_eq!(
        env["client2"].introspect().player_leave_events[0].player,
        client1_me
    );
    assert_eq!(env["client3"].introspect().player_leave_events.len(), 1);
    assert_eq!(
        env["client3"].introspect().player_leave_events[0].player,
        client1_me
    );

    env["client2"].network().stop();
    env.flush_network();

    assert_eq!(env["client3"].introspect().player_leave_events.len(), 2);
    assert_eq!(
        env["client3"].introspect().player_leave_events[1].player,
        client2_me
    );
}
