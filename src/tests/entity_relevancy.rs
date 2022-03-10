use super::common::prelude::*;
use crate::prelude::*;

// this variable should be even
const ENTITY_COUNT: u32 = 10;

// tests a few cases:
// - not spawning an entity marked irrelevant on spawn
// - spawning the entity when it becomes relevant
// - despawning irrelevant entities
// - spawn->despawn->spawn->despawn
// TODO: this test is wearing too many hats, needs to be broken up
#[test]
fn set_relevancy() {
    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    for _ in 0..ENTITY_COUNT {
        env["server"].world().spawn().insert(NetworkEntity::new());
    }
    env.flush_network();

    assert_eq!(env["server"].world().entities().len(), ENTITY_COUNT);
    assert_eq!(env["client1"].world().entities().len(), ENTITY_COUNT);
    assert_eq!(env["client2"].world().entities().len(), ENTITY_COUNT);

    let mut network_entity_query = env["server"].world().query::<&NetworkEntity>();
    let network_entities: Vec<NetworkEntity> = network_entity_query
        .iter(&env["server"].world())
        .map(|e| *e)
        .collect();
    let client1_me = env["client1"].network().me().unwrap();
    for (i, network_entity) in network_entities.iter().enumerate() {
        env["server"]
            .server()
            .set_entity_relevant(*network_entity, client1_me, i % 2 == 0);
    }
    env.flush_network();

    assert_eq!(env["server"].world().entities().len(), ENTITY_COUNT);
    assert_eq!(env["client1"].world().entities().len(), ENTITY_COUNT / 2);
    assert_eq!(env["client2"].world().entities().len(), ENTITY_COUNT);
    let client2_me = env["client2"].network().me().unwrap();
    for (i, network_entity) in network_entities.iter().enumerate() {
        env["server"]
            .server()
            .set_entity_relevant(*network_entity, client1_me, true);
        env["server"]
            .server()
            .set_entity_relevant(*network_entity, client2_me, i % 2 == 0);
    }
    env.flush_network();

    assert_eq!(env["server"].world().entities().len(), ENTITY_COUNT);
    assert_eq!(env["client1"].world().entities().len(), ENTITY_COUNT);
    assert_eq!(env["client2"].world().entities().len(), ENTITY_COUNT / 2);
    env.flush_network();

    for (i, network_entity) in network_entities.iter().enumerate() {
        env["server"]
            .server()
            .set_entity_relevant(*network_entity, client1_me, i % 2 == 0);
    }
    env.flush_network();

    assert_eq!(env["server"].world().entities().len(), ENTITY_COUNT);
    assert_eq!(env["client1"].world().entities().len(), ENTITY_COUNT / 2);
    assert_eq!(env["client2"].world().entities().len(), ENTITY_COUNT / 2);
}

#[test]
fn ownership_overrides() {
    // if a player owns an entity, it forces relevancy

    let mut env = TestEnvironment::default();
    env.create_server_client("server");
    env.create_client("client", "server");
    env.flush_network();

    let client_me = env["client"].network().me().unwrap();
    let mut network_entities = vec![];
    for _ in 0..ENTITY_COUNT {
        let network_entity = NetworkEntity::new();
        network_entities.push(network_entity);
        env["server"].world().spawn().insert(network_entity);
        env["server"]
            .server()
            .set_entity_relevant(network_entity, client_me, false);
        env["server"]
            .server()
            .set_entity_owner(network_entity, Some(client_me));
    }
    env.flush_network();

    assert_eq!(env["server"].world().entities().len(), ENTITY_COUNT);
    assert_eq!(env["client"].world().entities().len(), ENTITY_COUNT);

    for network_entity in network_entities.iter() {
        env["server"]
            .server()
            .set_entity_owner(*network_entity, None);
    }
    env.flush_network();

    assert_eq!(env["server"].world().entities().len(), ENTITY_COUNT);
    assert_eq!(env["client"].world().entities().len(), 0);
}

#[test]
fn server_overrides_relevancy() {
    // Entities cannot be irrelevant on server

    let mut env = TestEnvironment::default();
    env.create_server_client("server");
    env.flush_network();

    let mut network_entities = vec![];
    for _ in 0..ENTITY_COUNT {
        let network_entity = NetworkEntity::new();
        network_entities.push(network_entity);
        env["server"].world().spawn().insert(network_entity);
        let me = env["server"].network().me().unwrap();
        env["server"]
            .server()
            .set_entity_relevant(network_entity, me, false);
    }
    env.flush_network();

    assert_eq!(env["server"].world().entities().len(), ENTITY_COUNT);
}
