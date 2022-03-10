use super::common::prelude::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn get_entity_from_network_entity(app: &mut App, network_entity: NetworkEntity) -> Entity {
    let mut query = app.world.query::<(Entity, &NetworkEntity)>();
    for (entity, ne) in query.iter(&app.world) {
        if *ne == network_entity {
            return entity;
        }
    }
    panic!();
}

#[test]
fn server_client_send_as_owner() {
    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    let network_entity = NetworkEntity::new();
    let entity = env["server"].world().spawn().insert(network_entity).id();
    let server_me = env["server"].network().me();
    env["server"]
        .server()
        .set_entity_owner(network_entity, server_me);
    env.flush_network();

    env["server"]
        .client()
        .send_to_entity(network_entity, TestGameEvent { foo: "bar".into() });
    env.flush_network();

    let server_me = env["server"].network().me().unwrap();
    assert_eq!(env["server"].introspect().test_entity_events.len(), 1);
    assert_eq!(
        env["server"].introspect().test_entity_events[0].from,
        Some(server_me)
    );
    assert_eq!(
        env["server"].introspect().test_entity_events[0].entity,
        entity
    );
    assert_eq!(
        env["server"].introspect().test_entity_events[0].data.foo,
        "bar"
    );
    assert_eq!(env["client1"].introspect().test_entity_events.len(), 0);
    assert_eq!(env["client1"].introspect().test_entity_events.len(), 0);
}

#[test]
fn client_send_as_owner() {
    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    let client1_me = env["client1"].network().me().unwrap();
    let network_entity = NetworkEntity::new();
    env["server"].world().spawn().insert(network_entity);
    env["server"]
        .server()
        .set_entity_owner(network_entity, Some(client1_me));
    env.flush_network();

    env["client1"]
        .client()
        .send_to_entity(network_entity, TestGameEvent { foo: "bar".into() });
    env.flush_network();

    assert_eq!(env["server"].introspect().test_entity_events.len(), 0);
    assert_eq!(env["client1"].introspect().test_entity_events.len(), 1);
    assert_eq!(
        env["client1"].introspect().test_entity_events[0].from,
        Some(client1_me),
    );
    let entity = get_entity_from_network_entity(env["client1"].app(), network_entity);
    assert_eq!(
        env["client1"].introspect().test_entity_events[0].entity,
        entity
    );
    assert_eq!(
        env["client1"].introspect().test_entity_events[0].data.foo,
        "bar"
    );
    assert_eq!(env["client2"].introspect().test_entity_events.len(), 0);
}

#[test]
fn server_client_send_as_non_owner() {
    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    let client1_me = env["client1"].network().me().unwrap();
    let network_entity = NetworkEntity::new();
    let entity = env["server"].world().spawn().insert(network_entity).id();
    env["server"]
        .server()
        .set_entity_owner(network_entity, Some(client1_me));
    env.flush_network();

    env["server"]
        .client()
        .send_to_entity(network_entity, TestGameEvent { foo: "bar".into() });
    env.flush_network();

    let server_me = env["server"].network().me().unwrap();
    assert_eq!(env["server"].introspect().test_entity_events.len(), 0);
    assert_eq!(env["client1"].introspect().test_entity_events.len(), 1);
    assert_eq!(
        env["client1"].introspect().test_entity_events[0].from,
        Some(server_me),
    );
    assert_eq!(
        env["client1"].introspect().test_entity_events[0].entity,
        entity
    );
    assert_eq!(
        env["client1"].introspect().test_entity_events[0].data.foo,
        "bar"
    );
    assert_eq!(env["client2"].introspect().test_entity_events.len(), 0);
}

#[test]
fn client_send_as_non_owner() {
    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    let network_entity = NetworkEntity::new();
    env["server"].world().spawn().insert(network_entity);
    env.flush_network();

    env["client1"]
        .client()
        .send_to_entity(network_entity, TestGameEvent { foo: "bar".into() });
    env.flush_network();

    let client1_me = env["client1"].network().me().unwrap();
    assert_eq!(env["server"].introspect().test_entity_events.len(), 1);
    assert_eq!(
        env["server"].introspect().test_entity_events[0].from,
        Some(client1_me),
    );
    let entity = get_entity_from_network_entity(env["server"].app(), network_entity);
    assert_eq!(
        env["server"].introspect().test_entity_events[0].entity,
        entity
    );
    assert_eq!(
        env["server"].introspect().test_entity_events[0].data.foo,
        "bar"
    );
    assert_eq!(env["client1"].introspect().test_entity_events.len(), 0);
    assert_eq!(env["client2"].introspect().test_entity_events.len(), 0);
}
