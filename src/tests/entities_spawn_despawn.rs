use super::common::prelude::*;
use crate::prelude::*;
use bevy::prelude::*;

const ENTITY_COUNT: u32 = 3;

fn get_network_entity_ids(app: &mut App) -> Vec<String> {
    let mut query = app.world.query::<&NetworkEntity>();
    let mut ids: Vec<String> = query.iter(&app.world).map(|e| e.0.to_string()).collect();
    ids.sort_by(|a, b| a.partial_cmp(b).unwrap());
    ids
}

fn assert_same_entities(env: &mut TestEnvironment, app1: &str, app2: &str) {
    let entity_ids1 = get_network_entity_ids(env[app1].app());
    let entity_ids2 = get_network_entity_ids(env[app2].app());
    assert_eq!(entity_ids1, entity_ids2);
}

#[test]
fn spawn_on_join() {
    // Test that entities are spawned on client when they join

    let mut env = TestEnvironment::default();

    env.create_server("server");
    for _ in 0..ENTITY_COUNT {
        env["server"].world().spawn().insert(NetworkEntity::new());
    }

    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    assert_same_entities(&mut env, "server", "client1");
    assert_same_entities(&mut env, "server", "client2");
}

#[test]
fn spawn_after_join() {
    // Test that entities are spawned on client after they join

    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    for _ in 0..ENTITY_COUNT {
        env["server"].world().spawn().insert(NetworkEntity::new());
    }
    env.flush_network();

    assert_same_entities(&mut env, "server", "client1");
    assert_same_entities(&mut env, "server", "client2");
}

#[test]
fn despawn() {
    // Test that entities are despawned on clients

    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    for _ in 0..ENTITY_COUNT {
        env["server"].world().spawn().insert(NetworkEntity::new());
    }
    env.flush_network();

    assert_same_entities(&mut env, "server", "client1");
    assert_same_entities(&mut env, "server", "client2");

    let mut query = env["server"]
        .world()
        .query_filtered::<Entity, With<NetworkEntity>>();
    let entities: Vec<Entity> = query.iter(&env["server"].world()).map(|e| e).collect();
    for entity in entities.iter() {
        env["server"].world().entity_mut(*entity).despawn();
    }
    env.flush_network();

    assert_eq!(env["client1"].world().entities().len(), 0);
    assert_eq!(env["client2"].world().entities().len(), 0);
}

#[test]
fn despawn_on_stop() {
    // Test that entities are despawned on clients on client.stop()

    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    for _ in 0..ENTITY_COUNT {
        env["server"].world().spawn().insert(NetworkEntity::new());
    }
    env.flush_network();

    assert_same_entities(&mut env, "server", "client1");
    assert_same_entities(&mut env, "server", "client2");

    env["client1"].network().stop();
    env.flush_network();
    assert_eq!(env["client1"].world().entities().len(), 0);
    assert_eq!(env["client2"].world().entities().len(), ENTITY_COUNT);

    env["client2"].network().stop();
    env.flush_network();
    assert_eq!(env["client1"].world().entities().len(), 0);
    assert_eq!(env["client2"].world().entities().len(), 0);
}

#[test]
pub fn despawn_on_disconnect() {
    // Test that entities are despawned on clients on server.stop()

    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    for _ in 0..ENTITY_COUNT {
        env["server"].world().spawn().insert(NetworkEntity::new());
    }
    env.flush_network();

    assert_same_entities(&mut env, "server", "client1");
    assert_same_entities(&mut env, "server", "client2");

    env["server"].network().stop();
    env.flush_network();
    assert_eq!(env["client1"].world().entities().len(), 0);
    assert_eq!(env["client2"].world().entities().len(), 0);
}
