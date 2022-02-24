use super::common::prelude::*;
use bevy::prelude::*;

/// Player ordering should be the same between server/clients
#[test]
fn ensure_order() {
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
    let server_players = server_app.network().players();
    let client1_players = client1_app.network().players();
    let client2_players = client2_app.network().players();
    assert_eq!(server_players.len(), 2);
    assert_eq!(client1_players.len(), 2);
    assert_eq!(client2_players.len(), 2);
    assert_eq!(server_players[0], client1_players[0]);
    assert_eq!(server_players[0], client2_players[0]);
    assert_eq!(server_players[1], client1_players[1]);
    assert_eq!(server_players[1], client2_players[1]);
}
