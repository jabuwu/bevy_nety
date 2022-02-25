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
        .set_entity_owner(network_entity, client1_app.network().me());
    flush_network(vec![&mut server_app, &mut client1_app, &mut client2_app]);
    get_entity_owner(&mut client1_app, entity).send(TestGameEvent { foo: "bar".into() });
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

#[test]
fn send_from_server_client() {
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
    let server_me = server_app.network().me();
    server_app
        .network_mut()
        .server_mut()
        .unwrap()
        .set_entity_owner(network_entity, server_me);
    flush_network(vec![&mut server_app, &mut client_app]);
    get_entity_owner(&mut server_app, entity).send(TestGameEvent { foo: "bar".into() });
    flush_network(vec![&mut server_app, &mut client_app]);
    assert_eq!(server_app.introspect().test_entity_events.len(), 0);
    assert_eq!(client_app.introspect().test_entity_events.len(), 1);
    assert_eq!(client_app.introspect().test_entity_events[0].entity, entity);
    assert_eq!(client_app.introspect().test_entity_events[0].from, None);
    assert_eq!(
        client_app.introspect().test_entity_events[0].data.foo,
        "bar"
    );
}
