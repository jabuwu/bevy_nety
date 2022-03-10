use super::common::prelude::*;

#[test]
fn ensure_order() {
    // Player ordering should be the same between server/clients
    // This test has the side effect of also testing that player IDs are identical between peers
    let mut env = TestEnvironment::default();
    env.create_server("server");
    env.create_client("client1", "server");
    env.create_client("client2", "server");
    env.flush_network();
    let server_players = env["server"].network().players();
    let client1_players = env["client1"].network().players();
    let client2_players = env["client2"].network().players();
    assert_eq!(server_players.len(), 2);
    assert_eq!(client1_players.len(), 2);
    assert_eq!(client2_players.len(), 2);
    assert_eq!(server_players[0], client1_players[0]);
    assert_eq!(server_players[0], client2_players[0]);
    assert_eq!(server_players[1], client1_players[1]);
    assert_eq!(server_players[1], client2_players[1]);
}
