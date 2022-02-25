use super::common::prelude::*;
use crate::prelude::*;
use bevy::prelude::*;

#[test]
fn server_client_send_to_entity() {
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
    let network_entity = NetworkEntity::new();
    let entity = server_app.world.spawn().insert(network_entity).id();
    server_app
        .network_mut()
        .server_mut()
        .unwrap()
        .send_to_entity(network_entity, TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.introspect().test_entity_events.len(), 1);
    assert_eq!(server_app.introspect().test_entity_events[0].entity, entity);
    assert_eq!(server_app.introspect().test_entity_events[0].from, None);
    assert_eq!(
        server_app.introspect().test_entity_events[0].data.foo,
        "bar"
    );
    assert_eq!(client_app.introspect().test_entity_events.len(), 1);
    assert_eq!(client_app.introspect().test_entity_events[0].entity, entity);
    assert_eq!(client_app.introspect().test_entity_events[0].from, None);
    assert_eq!(
        client_app.introspect().test_entity_events[0].data.foo,
        "bar"
    );
}

#[test]
fn server_client_send_to_entity_receive_from_irrelevant() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut app = App::new();
    app.setup_for_tests();
    app.network_mut()
        .start_server_client(vec![pseudo_net.create_host()]);
    let server_me = app.network().me().unwrap();
    flush_network(vec![&mut app]);
    let network_entity = NetworkEntity::new();
    let entity = app.world.spawn().insert(network_entity).id();
    app.network_mut()
        .server_mut()
        .unwrap()
        .set_entity_relevant(network_entity, server_me, false);
    flush_network(vec![&mut app]);
    app.network_mut()
        .server_mut()
        .unwrap()
        .send_to_entity(network_entity, TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut app]);
    assert_eq!(app.introspect().test_entity_events.len(), 1);
    assert_eq!(app.introspect().test_entity_events[0].entity, entity);
    assert_eq!(app.introspect().test_entity_events[0].from, None);
    assert_eq!(app.introspect().test_entity_events[0].data.foo, "bar");
}

#[test]
fn server_send_to_entity() {
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
    let entity = server_app.world.spawn().insert(network_entity).id();
    server_app
        .network_mut()
        .server_mut()
        .unwrap()
        .set_entity_relevant(network_entity, client1_app.network().me().unwrap(), false);
    server_app
        .network_mut()
        .server_mut()
        .unwrap()
        .send_to_entity(network_entity, TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    assert_eq!(server_app.introspect().test_entity_events.len(), 0);
    assert_eq!(client1_app.introspect().test_entity_events.len(), 0);
    assert_eq!(client2_app.introspect().test_entity_events.len(), 1);
    assert_eq!(
        client2_app.introspect().test_entity_events[0].entity,
        entity
    );
    assert_eq!(client2_app.introspect().test_entity_events[0].from, None);
    assert_eq!(
        client2_app.introspect().test_entity_events[0].data.foo,
        "bar"
    );
}
