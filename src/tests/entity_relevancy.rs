use super::common::prelude::*;
use crate::prelude::*;
use bevy::prelude::*;

// this variable should be even
const ENTITY_COUNT: u32 = 10;

// tests a few cases:
// - not spawning an entity marked irrelevant on spawn
// - spawning the entity when it becomes relevant
// - despawning irrelevant entities
// - spawn->despawn->spawn->despawn
#[test]
fn set_relevancy() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client1_app = App::new();
    let mut client2_app = App::new();
    server_app.setup_for_tests();
    client1_app.setup_for_tests();
    client2_app.setup_for_tests();
    server_app
        .network_mut()
        .start_server_client(vec![pseudo_net.create_host()]);
    client1_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    client2_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    for _ in 0..ENTITY_COUNT {
        server_app.world.spawn().insert(NetworkEntity::new());
    }
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.world.entities().len(), ENTITY_COUNT);
    assert_eq!(client1_app.world.entities().len(), ENTITY_COUNT);
    assert_eq!(client2_app.world.entities().len(), ENTITY_COUNT);
    let mut network_entity_query = server_app.world.query::<&NetworkEntity>();
    let network_entities: Vec<NetworkEntity> = network_entity_query
        .iter(&server_app.world)
        .map(|e| *e)
        .collect();
    for (i, network_entity) in network_entities.iter().enumerate() {
        server_app
            .network_mut()
            .server_mut()
            .unwrap()
            .set_entity_relevant(
                *network_entity,
                client1_app.network().me().unwrap(),
                i % 2 == 0,
            );
    }
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.world.entities().len(), ENTITY_COUNT);
    assert_eq!(client1_app.world.entities().len(), ENTITY_COUNT / 2);
    assert_eq!(client2_app.world.entities().len(), ENTITY_COUNT);
    for (i, network_entity) in network_entities.iter().enumerate() {
        server_app
            .network_mut()
            .server_mut()
            .unwrap()
            .set_entity_relevant(*network_entity, client1_app.network().me().unwrap(), true);
        server_app
            .network_mut()
            .server_mut()
            .unwrap()
            .set_entity_relevant(
                *network_entity,
                client2_app.network().me().unwrap(),
                i % 2 == 0,
            );
    }
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.world.entities().len(), ENTITY_COUNT);
    assert_eq!(client1_app.world.entities().len(), ENTITY_COUNT);
    assert_eq!(client2_app.world.entities().len(), ENTITY_COUNT / 2);
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    for (i, network_entity) in network_entities.iter().enumerate() {
        server_app
            .network_mut()
            .server_mut()
            .unwrap()
            .set_entity_relevant(
                *network_entity,
                client1_app.network().me().unwrap(),
                i % 2 == 0,
            );
    }
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.world.entities().len(), ENTITY_COUNT);
    assert_eq!(client1_app.world.entities().len(), ENTITY_COUNT / 2);
    assert_eq!(client2_app.world.entities().len(), ENTITY_COUNT / 2);
}

// if a player owns an entity, it forces relevancy
#[test]
fn ownership_overrides() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client_app = App::new();
    server_app.setup_for_tests();
    client_app.setup_for_tests();
    server_app
        .network_mut()
        .start_server_client(vec![pseudo_net.create_host()]);
    client_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    flush_network(vec![&mut server_app, &mut client_app]);
    let mut network_entities = vec![];
    for _ in 0..ENTITY_COUNT {
        let network_entity = NetworkEntity::new();
        network_entities.push(network_entity);
        server_app.world.spawn().insert(network_entity);
        server_app
            .network_mut()
            .server_mut()
            .unwrap()
            .set_entity_relevant(network_entity, client_app.network().me().unwrap(), false);
        server_app
            .network_mut()
            .server_mut()
            .unwrap()
            .set_entity_owner(network_entity, Some(client_app.network().me().unwrap()));
    }
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.world.entities().len(), ENTITY_COUNT);
    assert_eq!(client_app.world.entities().len(), ENTITY_COUNT);
    for network_entity in network_entities.iter() {
        server_app
            .network_mut()
            .server_mut()
            .unwrap()
            .set_entity_owner(*network_entity, None);
    }
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.world.entities().len(), ENTITY_COUNT);
    assert_eq!(client_app.world.entities().len(), 0);
}

#[test]
fn server_overrides() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut app = App::new();
    app.setup_for_tests();
    app.network_mut()
        .start_server_client(vec![pseudo_net.create_host()]);
    flush_network(vec![&mut app]);
    let mut network_entities = vec![];
    for _ in 0..ENTITY_COUNT {
        let network_entity = NetworkEntity::new();
        network_entities.push(network_entity);
        app.world.spawn().insert(network_entity);
        let me = app.network().me().unwrap();
        app.network_mut()
            .server_mut()
            .unwrap()
            .set_entity_relevant(network_entity, me, false);
    }
    flush_network(vec![&mut app]);
    assert_eq!(app.world.entities().len(), ENTITY_COUNT);
}
