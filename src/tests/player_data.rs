use super::common::prelude::*;
use crate::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct InvalidPlayerData;

#[test]
#[should_panic(
    expected = "The struct \"bevy_nety::tests::player_data::InvalidPlayerData\" has not been registered as networked player data."
)]
pub fn bad_struct_unregistered() {
    {
        let mut app = App::new();
        app.setup_for_tests();
        app.network_mut().set_my_player_data(InvalidPlayerData);
    }
}

#[test]
#[should_panic(
    expected = "The struct \"bevy_nety::tests::player_data::InvalidPlayerData\" has not been registered as networked player data."
)]
pub fn bad_struct_registered_as_event() {
    {
        let mut app = App::new();
        app.setup_for_tests();
        app.add_network_event::<InvalidPlayerData>();
        app.network_mut().set_my_player_data(InvalidPlayerData);
    }
}

#[test]
pub fn before_connect() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client1_app = App::new();
    let mut client2_app = App::new();
    server_app.setup_for_tests();
    client1_app.setup_for_tests();
    client2_app.setup_for_tests();
    server_app
        .network_mut()
        .set_my_player_data(TestPlayerData { name: "foo".into() });
    client1_app
        .network_mut()
        .set_my_player_data(TestPlayerData { name: "bar".into() });
    client2_app
        .network_mut()
        .set_my_player_data(TestPlayerData { name: "baz".into() });
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
    let server_players = server_app.network().players();
    let client1_players = client1_app.network().players();
    let client2_players = client2_app.network().players();
    assert_eq!(server_players.len(), 3);
    assert_eq!(client1_players.len(), 3);
    assert_eq!(client2_players.len(), 3);
    assert_eq!(
        server_app
            .network()
            .get_player_data::<TestPlayerData>(server_players[0])
            .name,
        "foo"
    );
    assert_eq!(
        server_app
            .network()
            .get_player_data::<TestPlayerData>(server_players[1])
            .name,
        "bar"
    );
    assert_eq!(
        server_app
            .network()
            .get_player_data::<TestPlayerData>(server_players[2])
            .name,
        "baz"
    );
    assert_eq!(
        server_app
            .network()
            .get_player_data::<TestPlayerData>(client1_players[0])
            .name,
        "foo"
    );
    assert_eq!(
        server_app
            .network()
            .get_player_data::<TestPlayerData>(client1_players[1])
            .name,
        "bar"
    );
    assert_eq!(
        server_app
            .network()
            .get_player_data::<TestPlayerData>(client1_players[2])
            .name,
        "baz"
    );
    assert_eq!(
        server_app
            .network()
            .get_player_data::<TestPlayerData>(client2_players[0])
            .name,
        "foo"
    );
    assert_eq!(
        server_app
            .network()
            .get_player_data::<TestPlayerData>(client2_players[1])
            .name,
        "bar"
    );
    assert_eq!(
        server_app
            .network()
            .get_player_data::<TestPlayerData>(client2_players[2])
            .name,
        "baz"
    );
}

// TODO: this probably shouldn't be a panic, or even complete failure
//       but for now this is how it works
#[test]
#[should_panic(expected = "Cannot set player data while connecting.")]
pub fn while_connecting() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client_app = App::new();
    server_app.setup_for_tests();
    client_app.setup_for_tests();
    server_app
        .network_mut()
        .start_server(vec![pseudo_net.create_host()]);
    let (connector, ..) = pseudo_net.create_connector().as_pending();
    client_app.network_mut().start_client(connector);
    client_app
        .network_mut()
        .set_my_player_data(TestPlayerData { name: "foo".into() })
}

// TODO: this probably shouldn't be a panic, or even complete failure
//       but for now this is how it works
#[test]
#[should_panic(expected = "Cannot set player data while connected.")]
pub fn while_connected() {
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
    flush_network(vec![&mut server_app, &mut client_app]);
    client_app
        .network_mut()
        .set_my_player_data(TestPlayerData { name: "foo".into() })
}
