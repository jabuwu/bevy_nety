use super::common::prelude::*;

#[test]
fn local_single() {
    let mut env = TestEnvironment::default();
    env.create_local("local");
    env.flush_network();
    let me = env["local"].network().me().unwrap();
    assert_eq!(env["local"].introspect().player_join_events.len(), 1);
    assert_eq!(env["local"].introspect().player_join_events[0].me, true);
    assert_eq!(
        env["local"].introspect().player_join_events[0].existing_player,
        false
    );
    assert_eq!(env["local"].introspect().player_join_events[0].player, me);
}

#[test]
fn server_client_single() {
    let mut env = TestEnvironment::default();
    env.create_server_client("server");
    env.flush_network();
    let me = env["server"].network().me().unwrap();
    assert_eq!(env["server"].introspect().player_join_events.len(), 1);
    assert_eq!(env["server"].introspect().player_join_events[0].me, true);
    assert_eq!(
        env["server"].introspect().player_join_events[0].existing_player,
        false
    );
    assert_eq!(env["server"].introspect().player_join_events[0].player, me);
}

#[test]
fn server_client_multiple() {
    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.flush_network();

    let server_me = env["server"].network().me().unwrap();
    assert_eq!(env["server"].introspect().player_join_events.len(), 1);
    assert_eq!(env["server"].introspect().player_join_events[0].me, true);
    assert_eq!(
        env["server"].introspect().player_join_events[0].existing_player,
        false
    );
    assert_eq!(
        env["server"].introspect().player_join_events[0].player,
        server_me
    );

    env.create_client("client", "server");
    env.flush_network();
    let client_me = env["client"].network().me().unwrap();
    assert_eq!(env["server"].introspect().player_join_events.len(), 2);
    assert_eq!(
        env["server"].introspect().player_join_events[1].player,
        client_me
    );
    assert_eq!(env["server"].introspect().player_join_events[1].me, false);
    assert_eq!(
        env["server"].introspect().player_join_events[1].existing_player,
        false
    );
}

#[test]
fn server_single() {
    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.flush_network();

    assert_eq!(env["server"].introspect().player_join_events.len(), 0);

    env.create_client("client", "server");
    env.flush_network();
    let client_me = env["client"].network().me().unwrap();
    assert_eq!(env["server"].introspect().player_join_events.len(), 1);
    assert_eq!(
        env["server"].introspect().player_join_events[0].player,
        client_me
    );
    assert_eq!(env["server"].introspect().player_join_events[0].me, false);
    assert_eq!(
        env["server"].introspect().player_join_events[0].existing_player,
        false
    );
}

#[test]
fn server_multiple() {
    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.flush_network();
    assert_eq!(env["server"].introspect().player_join_events.len(), 0);

    env.create_client("client1", "server");
    env.flush_network();
    let client1_me = env["client1"].network().me().unwrap();
    assert_eq!(env["server"].introspect().player_join_events.len(), 1);
    assert_eq!(
        env["server"].introspect().player_join_events[0].player,
        client1_me
    );
    assert_eq!(env["server"].introspect().player_join_events[0].me, false);
    assert_eq!(
        env["server"].introspect().player_join_events[0].existing_player,
        false
    );

    env.create_client("client2", "server");
    env.flush_network();
    let client2_me = env["client2"].network().me().unwrap();
    assert_eq!(env["server"].introspect().player_join_events.len(), 2);
    assert_eq!(
        env["server"].introspect().player_join_events[1].player,
        client2_me
    );
    assert_eq!(env["server"].introspect().player_join_events[1].me, false);
    assert_eq!(
        env["server"].introspect().player_join_events[1].existing_player,
        false
    );
}

#[test]
fn client_single() {
    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_app("client");
    env.flush_network();
    assert_eq!(env["client"].introspect().player_join_events.len(), 0);

    env.start_client("client", "server");
    env.flush_network();

    let client_me = env["client"].network().me().unwrap();
    assert_eq!(env["client"].introspect().player_join_events.len(), 1);
    assert_eq!(env["client"].introspect().player_join_events[0].me, true);
    assert_eq!(
        env["client"].introspect().player_join_events[0].existing_player,
        false
    );
    assert_eq!(
        env["client"].introspect().player_join_events[0].player,
        client_me
    );
}

#[test]
fn client_multiple() {
    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_app("client1");
    env.create_app("client2");
    env.flush_network();
    assert_eq!(env["client1"].introspect().player_join_events.len(), 0);
    assert_eq!(env["client2"].introspect().player_join_events.len(), 0);

    env.start_client("client1", "server");
    env.flush_network();

    let client1_me = env["client1"].network().me().unwrap();
    assert_eq!(env["client1"].introspect().player_join_events.len(), 1);
    assert_eq!(env["client1"].introspect().player_join_events[0].me, true);
    assert_eq!(
        env["client1"].introspect().player_join_events[0].existing_player,
        false
    );
    assert_eq!(
        env["client1"].introspect().player_join_events[0].player,
        client1_me
    );
    assert_eq!(env["client2"].introspect().player_join_events.len(), 0);

    env.start_client("client2", "server");
    env.flush_network();

    let client2_me = env["client2"].network().me().unwrap();
    assert_eq!(env["client1"].introspect().player_join_events.len(), 2);
    assert_eq!(env["client1"].introspect().player_join_events[1].me, false);
    assert_eq!(
        env["client1"].introspect().player_join_events[1].existing_player,
        false
    );
    assert_eq!(
        env["client1"].introspect().player_join_events[1].player,
        client2_me
    );
    assert_eq!(env["client2"].introspect().player_join_events.len(), 2);
    assert_eq!(env["client2"].introspect().player_join_events[0].me, false);
    assert_eq!(
        env["client2"].introspect().player_join_events[0].existing_player,
        true
    );
    assert_eq!(
        env["client2"].introspect().player_join_events[0].player,
        client1_me
    );
    assert_eq!(env["client2"].introspect().player_join_events[1].me, true);
    assert_eq!(
        env["client2"].introspect().player_join_events[1].existing_player,
        false
    );
    assert_eq!(
        env["client2"].introspect().player_join_events[1].player,
        client2_me
    );
}
