use super::common::prelude::*;
use crate::prelude::*;
use bevy::prelude::*;

// TODO: disallow messages force sent from non-owners

pub fn get_entity_owner(app: &mut App, entity: Entity) -> Mut<NetworkEntityOwner> {
    let mut query = app.world.query::<&mut NetworkEntityOwner>();
    query.get_mut(&mut app.world, entity).unwrap()
}

#[test]
fn send_from_client() {
    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    let network_entity = NetworkEntity::new();
    let entity = env["server"].world().spawn().insert(network_entity).id();
    let client1_me = env["client1"].network().me().unwrap();
    env["server"]
        .server()
        .set_entity_owner(network_entity, Some(client1_me));
    env.flush_network();

    get_entity_owner(env["client1"].app(), entity).send(TestGameEvent { foo: "bar".into() });
    env.flush_network();

    assert_eq!(env["server"].introspect().test_entity_events.len(), 0);
    assert_eq!(env["client1"].introspect().test_entity_events.len(), 0);
    assert_eq!(env["client2"].introspect().test_entity_events.len(), 1);
    assert_eq!(
        env["client2"].introspect().test_entity_events[0].entity,
        entity
    );
    assert_eq!(env["client2"].introspect().test_entity_events[0].from, None);
    assert_eq!(
        env["client2"].introspect().test_entity_events[0].data.foo,
        "bar"
    );
}

#[test]
fn send_from_server_client() {
    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.create_client("client", "server");
    env.flush_network();

    let network_entity = NetworkEntity::new();
    let entity = env["server"].world().spawn().insert(network_entity).id();
    let server_me = env["server"].network().me().unwrap();
    env["server"]
        .server()
        .set_entity_owner(network_entity, Some(server_me));
    env.flush_network();

    get_entity_owner(env["server"].app(), entity).send(TestGameEvent { foo: "bar".into() });
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
