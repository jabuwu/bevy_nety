// TODO: fail to set owner if player doesn't exist?
// TODO: reset owner on player disconnect

use super::common::prelude::*;
use crate::prelude::*;
use bevy::prelude::*;

// this variable should be even
const ENTITY_COUNT: u32 = 10;

pub fn network_entity_owner_count(app: &mut App) -> u32 {
    let mut network_entity_owner_query = app.world.query::<&NetworkEntityOwner>();
    network_entity_owner_query
        .iter(&app.world)
        .map(|e| e)
        .count() as u32
}

#[test]
fn server_owner() {
    let mut env = TestEnvironment::default();

    env.create_server("server");
    env.create_client("client", "server");
    env.flush_network();

    for _ in 0..ENTITY_COUNT {
        env["server"].world().spawn().insert(NetworkEntity::new());
    }
    env.flush_network();
    assert_eq!(
        network_entity_owner_count(env["server"].app()),
        ENTITY_COUNT
    );
}

#[test]
fn server_client_owner() {
    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.create_client("client", "server");
    env.flush_network();

    for _ in 0..ENTITY_COUNT {
        env["server"].world().spawn().insert(NetworkEntity::new());
    }
    env.flush_network();
    assert_eq!(
        network_entity_owner_count(env["server"].app()),
        ENTITY_COUNT
    );
}

#[test]
fn owner_on_spawn() {
    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    let client1_me = env["client1"].network().me().unwrap();
    for i in 0..ENTITY_COUNT {
        let network_entity = NetworkEntity::new();
        env["server"].world().spawn().insert(network_entity);
        if i % 2 == 0 {
            env["server"]
                .server()
                .set_entity_owner(network_entity, Some(client1_me))
        }
    }
    env.flush_network();

    assert_eq!(
        network_entity_owner_count(env["server"].app()),
        ENTITY_COUNT / 2
    );
    assert_eq!(
        network_entity_owner_count(env["client1"].app()),
        ENTITY_COUNT / 2
    );
    assert_eq!(network_entity_owner_count(env["client2"].app()), 0);
}

#[test]
fn owner_change() {
    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    let client1_me = env["client1"].network().me().unwrap();
    let mut network_entities = vec![];
    for i in 0..ENTITY_COUNT {
        let network_entity = NetworkEntity::new();
        network_entities.push(network_entity);
        env["server"].world().spawn().insert(network_entity);
        if i % 2 == 0 {
            env["server"]
                .server()
                .set_entity_owner(network_entity, Some(client1_me))
        }
    }
    env.flush_network();

    assert_eq!(
        network_entity_owner_count(env["server"].app()),
        ENTITY_COUNT / 2
    );
    assert_eq!(
        network_entity_owner_count(env["client1"].app()),
        ENTITY_COUNT / 2
    );
    assert_eq!(network_entity_owner_count(env["client2"].app()), 0);

    let client2_me = env["client2"].network().me().unwrap();
    for (i, network_entity) in network_entities.iter().enumerate() {
        if i % 2 == 0 {
            env["server"]
                .server()
                .set_entity_owner(*network_entity, Some(client2_me))
        }
    }
    env.flush_network();

    assert_eq!(
        network_entity_owner_count(env["server"].app()),
        ENTITY_COUNT / 2
    );
    assert_eq!(network_entity_owner_count(env["client1"].app()), 0);
    assert_eq!(
        network_entity_owner_count(env["client2"].app()),
        ENTITY_COUNT / 2
    );
    for network_entity in network_entities.iter() {
        env["server"]
            .server()
            .set_entity_owner(*network_entity, None)
    }
    env.flush_network();

    assert_eq!(
        network_entity_owner_count(env["server"].app()),
        ENTITY_COUNT
    );
    assert_eq!(network_entity_owner_count(env["client1"].app()), 0);
    assert_eq!(network_entity_owner_count(env["client2"].app()), 0);
}

#[test]
fn owner_multiple_sets() {
    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    let client1_me = env["client1"].network().me().unwrap();
    let client2_me = env["client2"].network().me().unwrap();
    let mut network_entities = vec![];
    for _ in 0..ENTITY_COUNT {
        let network_entity = NetworkEntity::new();
        network_entities.push(network_entity);
        env["server"].world().spawn().insert(network_entity);
        env["server"]
            .server()
            .set_entity_owner(network_entity, Some(client1_me));
        env["server"]
            .server()
            .set_entity_owner(network_entity, Some(client2_me));
    }
    env.flush_network();

    assert_eq!(network_entity_owner_count(env["server"].app()), 0);
    assert_eq!(network_entity_owner_count(env["client1"].app()), 0);
    assert_eq!(
        network_entity_owner_count(env["client2"].app()),
        ENTITY_COUNT
    );
}

#[test]
fn reset_owner_on_player_leave() {
    let mut env = TestEnvironment::default();

    env.create_server_client("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();

    let client1_me = env["client1"].network().me().unwrap();
    let client2_me = env["client2"].network().me().unwrap();
    for i in 0..ENTITY_COUNT {
        let network_entity = NetworkEntity::new();
        env["server"].world().spawn().insert(network_entity);
        if i % 2 == 0 {
            env["server"]
                .server()
                .set_entity_owner(network_entity, Some(client1_me))
        } else {
            env["server"]
                .server()
                .set_entity_owner(network_entity, Some(client2_me))
        }
    }
    env.flush_network();

    assert_eq!(network_entity_owner_count(env["server"].app()), 0);
    assert_eq!(
        network_entity_owner_count(env["client1"].app()),
        ENTITY_COUNT / 2
    );
    assert_eq!(
        network_entity_owner_count(env["client2"].app()),
        ENTITY_COUNT / 2
    );

    env["client1"].network().stop();
    env.flush_network();

    assert_eq!(
        network_entity_owner_count(env["server"].app()),
        ENTITY_COUNT / 2
    );
    assert_eq!(network_entity_owner_count(env["client1"].app()), 0);
    assert_eq!(
        network_entity_owner_count(env["client2"].app()),
        ENTITY_COUNT / 2
    );

    env["client2"].network().stop();
    env.flush_network();

    assert_eq!(
        network_entity_owner_count(env["server"].app()),
        ENTITY_COUNT
    );
    assert_eq!(network_entity_owner_count(env["client1"].app()), 0);
    assert_eq!(network_entity_owner_count(env["client2"].app()), 0);
}
