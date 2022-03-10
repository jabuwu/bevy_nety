use super::common::prelude::*;
use crate::prelude::*;

// Server should send entity events after relevancy updates
// The following tests help ensure this:
// - server_client_send_to_entity_receive_from_irrelevant
// - server_send_to_entity_ignore_irrelevant
// Might need more checks to guarantee this

#[test]
fn server_client_send_to_entity() {
    // Server receives entity messages
    // Client does too if its relevant, in this case it is

    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.create_client("client", "server");
    env.flush_network();

    let network_entity = NetworkEntity::new();
    let entity = env["server"].world().spawn().insert(network_entity).id();
    env.flush_network();
    env["server"]
        .server()
        .send_to_entity(network_entity, TestGameEvent { foo: "bar".into() });
    env.flush_network();

    assert_eq!(env["server"].introspect().test_entity_events.len(), 1);
    assert_eq!(
        env["server"].introspect().test_entity_events[0].entity,
        entity
    );
    assert_eq!(env["server"].introspect().test_entity_events[0].from, None);
    assert_eq!(
        env["server"].introspect().test_entity_events[0].data.foo,
        "bar"
    );
    assert_eq!(env["client"].introspect().test_entity_events.len(), 1);
    assert_eq!(
        env["client"].introspect().test_entity_events[0].entity,
        entity
    );
    assert_eq!(env["client"].introspect().test_entity_events[0].from, None);
    assert_eq!(
        env["client"].introspect().test_entity_events[0].data.foo,
        "bar"
    );
}

#[test]
fn server_client_send_to_entity_receive_from_irrelevant() {
    // Server always receives entity events, even if irrelevant

    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.flush_network();

    let server_me = env["server"].network().me().unwrap();
    let network_entity = NetworkEntity::new();
    let entity = env["server"].world().spawn().insert(network_entity).id();
    env["server"]
        .server()
        .set_entity_relevant(network_entity, server_me, false);
    env["server"]
        .server()
        .send_to_entity(network_entity, TestGameEvent { foo: "bar".into() });
    env.flush_network();

    assert_eq!(env["server"].introspect().test_entity_events.len(), 1);
    assert_eq!(
        env["server"].introspect().test_entity_events[0].entity,
        entity
    );
    assert_eq!(env["server"].introspect().test_entity_events[0].from, None);
    assert_eq!(
        env["server"].introspect().test_entity_events[0].data.foo,
        "bar"
    );
}

#[test]
fn server_send_to_entity() {
    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_client("client", "server");
    env.flush_network();

    let network_entity = NetworkEntity::new();
    let entity = env["server"].world().spawn().insert(network_entity).id();
    env.flush_network();
    env["server"]
        .server()
        .send_to_entity(network_entity, TestGameEvent { foo: "bar".into() });
    env.flush_network();

    assert_eq!(env["server"].introspect().test_entity_events.len(), 0);
    assert_eq!(env["client"].introspect().test_entity_events.len(), 1);
    assert_eq!(
        env["client"].introspect().test_entity_events[0].entity,
        entity
    );
    assert_eq!(env["client"].introspect().test_entity_events[0].from, None);
    assert_eq!(
        env["client"].introspect().test_entity_events[0].data.foo,
        "bar"
    );
}

#[test]
fn server_send_to_entity_ignore_irrelevant() {
    // set to irrelevant and then send entity event
    // client should not receive the event

    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_client("client", "server");
    env.flush_network();

    let client_me = env["client"].network().me().unwrap();
    let network_entity = NetworkEntity::new();
    env["server"].world().spawn().insert(network_entity).id();
    env.flush_network();
    env["server"]
        .server()
        .set_entity_relevant(network_entity, client_me, false);
    env["server"]
        .server()
        .send_to_entity(network_entity, TestGameEvent { foo: "bar".into() });
    env.flush_network();

    assert_eq!(env["server"].introspect().test_entity_events.len(), 0);
    assert_eq!(env["client"].introspect().test_entity_events.len(), 0);
}
