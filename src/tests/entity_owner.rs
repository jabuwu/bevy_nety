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
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client_app = App::new();
    server_app.setup_for_tests();
    client_app.setup_for_tests();
    server_app
        .network_mut()
        .start_server(vec![pseudo_net.create_host()]);
    client_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    for _ in 0..ENTITY_COUNT {
        server_app.world.spawn().insert(NetworkEntity::new());
    }
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(network_entity_owner_count(&mut server_app), ENTITY_COUNT);
}

#[test]
fn server_client_owner() {
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
    for _ in 0..ENTITY_COUNT {
        server_app.world.spawn().insert(NetworkEntity::new());
    }
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(network_entity_owner_count(&mut server_app), ENTITY_COUNT);
}

#[test]
fn owner_on_spawn() {
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
    for i in 0..ENTITY_COUNT {
        let network_entity = NetworkEntity::new();
        server_app.world.spawn().insert(network_entity);
        if i % 2 == 0 {
            server_app
                .network_mut()
                .server_mut()
                .unwrap()
                .set_entity_owner(network_entity, Some(client1_app.network().me().unwrap()))
        }
    }
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(
        network_entity_owner_count(&mut server_app),
        ENTITY_COUNT / 2
    );
    assert_eq!(
        network_entity_owner_count(&mut client1_app),
        ENTITY_COUNT / 2
    );
    assert_eq!(network_entity_owner_count(&mut client2_app), 0);
}

#[test]
fn owner_change() {
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
    let mut network_entities = vec![];
    for i in 0..ENTITY_COUNT {
        let network_entity = NetworkEntity::new();
        network_entities.push(network_entity);
        server_app.world.spawn().insert(network_entity);
        if i % 2 == 0 {
            server_app
                .network_mut()
                .server_mut()
                .unwrap()
                .set_entity_owner(network_entity, Some(client1_app.network().me().unwrap()))
        }
    }
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(
        network_entity_owner_count(&mut server_app),
        ENTITY_COUNT / 2
    );
    assert_eq!(
        network_entity_owner_count(&mut client1_app),
        ENTITY_COUNT / 2
    );
    assert_eq!(network_entity_owner_count(&mut client2_app), 0);
    for (i, network_entity) in network_entities.iter().enumerate() {
        if i % 2 == 0 {
            server_app
                .network_mut()
                .server_mut()
                .unwrap()
                .set_entity_owner(*network_entity, Some(client2_app.network().me().unwrap()))
        }
    }
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(
        network_entity_owner_count(&mut server_app),
        ENTITY_COUNT / 2
    );
    assert_eq!(network_entity_owner_count(&mut client1_app), 0);
    assert_eq!(
        network_entity_owner_count(&mut client2_app),
        ENTITY_COUNT / 2
    );
    for network_entity in network_entities.iter() {
        server_app
            .network_mut()
            .server_mut()
            .unwrap()
            .set_entity_owner(*network_entity, None)
    }
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(network_entity_owner_count(&mut server_app), ENTITY_COUNT);
    assert_eq!(network_entity_owner_count(&mut client1_app), 0);
    assert_eq!(network_entity_owner_count(&mut client2_app), 0);
}

#[test]
fn owner_multiple_sets() {
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
    let mut network_entities = vec![];
    for _ in 0..ENTITY_COUNT {
        let network_entity = NetworkEntity::new();
        network_entities.push(network_entity);
        server_app.world.spawn().insert(network_entity);
        server_app
            .network_mut()
            .server_mut()
            .unwrap()
            .set_entity_owner(network_entity, Some(client1_app.network().me().unwrap()));
        server_app
            .network_mut()
            .server_mut()
            .unwrap()
            .set_entity_owner(network_entity, Some(client2_app.network().me().unwrap()));
    }
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(network_entity_owner_count(&mut server_app), 0);
    assert_eq!(network_entity_owner_count(&mut client1_app), 0);
    assert_eq!(network_entity_owner_count(&mut client2_app), ENTITY_COUNT);
}

#[test]
fn reset_owner_on_player_leave() {
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
    for i in 0..ENTITY_COUNT {
        let network_entity = NetworkEntity::new();
        server_app.world.spawn().insert(network_entity);
        if i % 2 == 0 {
            server_app
                .network_mut()
                .server_mut()
                .unwrap()
                .set_entity_owner(network_entity, Some(client1_app.network().me().unwrap()))
        } else {
            server_app
                .network_mut()
                .server_mut()
                .unwrap()
                .set_entity_owner(network_entity, Some(client2_app.network().me().unwrap()))
        }
    }
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(network_entity_owner_count(&mut server_app), 0);
    assert_eq!(
        network_entity_owner_count(&mut client1_app),
        ENTITY_COUNT / 2
    );
    assert_eq!(
        network_entity_owner_count(&mut client2_app),
        ENTITY_COUNT / 2
    );
    client1_app.network_mut().stop();
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(
        network_entity_owner_count(&mut server_app),
        ENTITY_COUNT / 2
    );
    assert_eq!(network_entity_owner_count(&mut client1_app), 0);
    assert_eq!(
        network_entity_owner_count(&mut client2_app),
        ENTITY_COUNT / 2
    );
    client2_app.network_mut().stop();
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(network_entity_owner_count(&mut server_app), ENTITY_COUNT);
    assert_eq!(network_entity_owner_count(&mut client1_app), 0);
    assert_eq!(network_entity_owner_count(&mut client2_app), 0);
}
