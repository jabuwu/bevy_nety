use super::common::prelude::*;
use crate::prelude::*;
use bevy::prelude::*;

const ENTITY_COUNT: u32 = 3;

fn get_network_entities_ids(app: &mut App) -> Vec<String> {
    let mut query = app.world.query::<&NetworkEntity>();
    let mut ids: Vec<String> = query.iter(&app.world).map(|e| e.0.to_string()).collect();
    ids.sort_by(|a, b| a.partial_cmp(b).unwrap());
    ids
}

#[test]
fn spawn_despawn() {
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
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    for _ in 0..ENTITY_COUNT {
        server_app.world.spawn().insert(NetworkEntity::new());
    }
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.world.entities().len(), ENTITY_COUNT);
    assert_eq!(client1_app.world.entities().len(), ENTITY_COUNT);
    client2_app
        .network_mut()
        .start_client(pseudo_net.create_connector().as_success());
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(client2_app.world.entities().len(), ENTITY_COUNT);
    let server_network_entities = get_network_entities_ids(&mut server_app);
    let client1_network_entities = get_network_entities_ids(&mut client1_app);
    let client2_network_entities = get_network_entities_ids(&mut client2_app);
    assert_eq!(server_network_entities, client1_network_entities);
    assert_eq!(server_network_entities, client2_network_entities);
    let mut query = server_app
        .world
        .query_filtered::<Entity, With<NetworkEntity>>();
    let entities: Vec<Entity> = query.iter(&server_app.world).map(|e| e).collect();
    for entity in entities.iter() {
        server_app.world.entity_mut(*entity).despawn();
    }
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.world.entities().len(), 0);
    assert_eq!(client1_app.world.entities().len(), 0);
    assert_eq!(client2_app.world.entities().len(), 0);
}

#[test]
fn despawn_on_stop() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client1_app = App::new();
    let mut client2_app = App::new();
    server_app.setup_for_tests();
    client1_app.setup_for_tests();
    client2_app.setup_for_tests();
    server_app
        .network_mut()
        .start_server(vec![pseudo_net.create_host()]);
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
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    client1_app.network_mut().stop();
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.world.entities().len(), ENTITY_COUNT);
    assert_eq!(client1_app.world.entities().len(), 0);
    assert_eq!(client2_app.world.entities().len(), ENTITY_COUNT);
    client2_app.network_mut().stop();
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.world.entities().len(), ENTITY_COUNT);
    assert_eq!(client2_app.world.entities().len(), 0);
}

#[test]
pub fn despawn_on_disconnect() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client1_app = App::new();
    let mut client2_app = App::new();
    server_app.setup_for_tests();
    client1_app.setup_for_tests();
    client2_app.setup_for_tests();
    server_app
        .network_mut()
        .start_server(vec![pseudo_net.create_host()]);
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
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    server_app.network_mut().stop();
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.world.entities().len(), 0);
    assert_eq!(client1_app.world.entities().len(), 0);
    assert_eq!(client2_app.world.entities().len(), 0);
}
