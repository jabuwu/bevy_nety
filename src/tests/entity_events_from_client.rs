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
    let network_entity = NetworkEntity::new();
    let entity = server_app.world.spawn().insert(network_entity).id();
    let server_me = server_app.network().me();
    server_app
        .network_mut()
        .server_mut()
        .unwrap()
        .set_entity_owner(network_entity, server_me);
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    server_app
        .network_mut()
        .client_mut()
        .unwrap()
        .send_to_entity(network_entity, TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.introspect().test_entity_events.len(), 1);
    assert_eq!(
        server_app.introspect().test_entity_events[0].from,
        Some(server_app.network().me().unwrap())
    );
    assert_eq!(server_app.introspect().test_entity_events[0].entity, entity);
    assert_eq!(
        server_app.introspect().test_entity_events[0].data.foo,
        "bar"
    );
    assert_eq!(client1_app.introspect().test_entity_events.len(), 0);
    assert_eq!(client2_app.introspect().test_entity_events.len(), 0);
}

#[test]
fn client_send_as_owner() {
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
    let network_entity = NetworkEntity::new();
    server_app.world.spawn().insert(network_entity);
    server_app
        .network_mut()
        .server_mut()
        .unwrap()
        .set_entity_owner(network_entity, client1_app.network().me());
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    client1_app
        .network_mut()
        .client_mut()
        .unwrap()
        .send_to_entity(network_entity, TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.introspect().test_entity_events.len(), 0);
    assert_eq!(client1_app.introspect().test_entity_events.len(), 1);
    assert_eq!(
        client1_app.introspect().test_entity_events[0].from,
        Some(client1_app.network().me().unwrap())
    );
    let entity = get_entity_from_network_entity(&mut client1_app, network_entity);
    assert_eq!(
        client1_app.introspect().test_entity_events[0].entity,
        entity
    );
    assert_eq!(
        client1_app.introspect().test_entity_events[0].data.foo,
        "bar"
    );
    assert_eq!(client2_app.introspect().test_entity_events.len(), 0);
}

#[test]
fn server_client_send_as_non_owner() {
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
    let network_entity = NetworkEntity::new();
    let entity = server_app.world.spawn().insert(network_entity).id();
    server_app
        .network_mut()
        .server_mut()
        .unwrap()
        .set_entity_owner(network_entity, client1_app.network().me());
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    server_app
        .network_mut()
        .client_mut()
        .unwrap()
        .send_to_entity(network_entity, TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.introspect().test_entity_events.len(), 0);
    assert_eq!(client1_app.introspect().test_entity_events.len(), 1);
    assert_eq!(
        client1_app.introspect().test_entity_events[0].from,
        Some(server_app.network().me().unwrap())
    );
    assert_eq!(
        client1_app.introspect().test_entity_events[0].entity,
        entity
    );
    assert_eq!(
        client1_app.introspect().test_entity_events[0].data.foo,
        "bar"
    );
    assert_eq!(client2_app.introspect().test_entity_events.len(), 0);
}

#[test]
fn client_send_as_non_owner() {
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
    let network_entity = NetworkEntity::new();
    server_app.world.spawn().insert(network_entity);
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    client1_app
        .network_mut()
        .client_mut()
        .unwrap()
        .send_to_entity(network_entity, TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.introspect().test_entity_events.len(), 1);
    assert_eq!(
        server_app.introspect().test_entity_events[0].from,
        Some(client1_app.network().me().unwrap())
    );
    let entity = get_entity_from_network_entity(&mut server_app, network_entity);
    assert_eq!(server_app.introspect().test_entity_events[0].entity, entity);
    assert_eq!(
        server_app.introspect().test_entity_events[0].data.foo,
        "bar"
    );
    assert_eq!(client1_app.introspect().test_entity_events.len(), 0);
    assert_eq!(client2_app.introspect().test_entity_events.len(), 0);
}
