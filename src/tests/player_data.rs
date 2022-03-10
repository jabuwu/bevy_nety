use super::common::prelude::*;
use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct InvalidPlayerData;

#[test]
#[should_panic(
    expected = "The struct \"bevy_nety::tests::player_data::InvalidPlayerData\" has not been registered as networked player data."
)]
pub fn bad_struct_unregistered() {
    let mut env = TestEnvironment::default();
    env.create_app("app");
    env["app"].network().set_my_player_data(InvalidPlayerData);
}

#[test]
#[should_panic(
    expected = "The struct \"bevy_nety::tests::player_data::InvalidPlayerData\" has not been registered as networked player data."
)]
pub fn bad_struct_registered_as_event() {
    let mut env = TestEnvironment::default();
    env.create_app("app");
    env["app"].app().add_network_event::<InvalidPlayerData>();
    env["app"].network().set_my_player_data(InvalidPlayerData);
}

#[test]
pub fn before_connect() {
    // Test setting player data before starting the server or connecting as a client.

    let mut env = TestEnvironment::default();

    env.create_app("server");
    env.create_app("client1");
    env.create_app("client2");

    env["server"]
        .network()
        .set_my_player_data(TestPlayerData { name: "foo".into() });
    env["client1"]
        .network()
        .set_my_player_data(TestPlayerData { name: "bar".into() });
    env["client2"]
        .network()
        .set_my_player_data(TestPlayerData { name: "baz".into() });

    env.start_server_client("server");
    env.flush_network();
    env.start_client("client1", "server");
    env.flush_network();
    env.start_client("client2", "server");
    env.flush_network();

    let server_players = env["server"].network().players();
    let client1_players = env["client1"].network().players();
    let client2_players = env["client2"].network().players();
    assert_eq!(server_players.len(), 3);
    assert_eq!(client1_players.len(), 3);
    assert_eq!(client2_players.len(), 3);
    assert_eq!(
        env["server"]
            .network()
            .get_player_data::<TestPlayerData>(server_players[0])
            .name,
        "foo"
    );
    assert_eq!(
        env["server"]
            .network()
            .get_player_data::<TestPlayerData>(server_players[1])
            .name,
        "bar"
    );
    assert_eq!(
        env["server"]
            .network()
            .get_player_data::<TestPlayerData>(server_players[2])
            .name,
        "baz"
    );
    assert_eq!(
        env["client1"]
            .network()
            .get_player_data::<TestPlayerData>(client1_players[0])
            .name,
        "foo"
    );
    assert_eq!(
        env["client1"]
            .network()
            .get_player_data::<TestPlayerData>(client1_players[1])
            .name,
        "bar"
    );
    assert_eq!(
        env["client1"]
            .network()
            .get_player_data::<TestPlayerData>(client1_players[2])
            .name,
        "baz"
    );
    assert_eq!(
        env["client2"]
            .network()
            .get_player_data::<TestPlayerData>(client2_players[0])
            .name,
        "foo"
    );
    assert_eq!(
        env["client2"]
            .network()
            .get_player_data::<TestPlayerData>(client2_players[1])
            .name,
        "bar"
    );
    assert_eq!(
        env["client2"]
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
    let mut env = TestEnvironment::default();
    env.create_server("server");
    env.create_client_pending("client", "server");
    env["client"]
        .network()
        .set_my_player_data(TestPlayerData { name: "foo".into() })
}

// TODO: this probably shouldn't be a panic, or even complete failure
//       but for now this is how it works
#[test]
#[should_panic(expected = "Cannot set player data while connected.")]
pub fn while_connected() {
    let mut env = TestEnvironment::default();
    env.create_server("server");
    env.create_client("client", "server");
    env.flush_network();
    env["client"]
        .network()
        .set_my_player_data(TestPlayerData { name: "foo".into() })
}
