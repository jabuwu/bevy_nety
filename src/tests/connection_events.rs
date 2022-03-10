use super::common::prelude::*;

// Test the following events:
// - NetworkConnectEvent
// - NetworkConnectingEvent
// - NetworkDisconnectEvent

#[test]
pub fn local_connect() {
    let mut env = TestEnvironment::default();

    // Create a local server and assert that connect event sends.
    // Local server does not send NetworkConnectingEvent
    env.create_local("local");
    env.flush_network();
    assert_eq!(env["local"].introspect().connect_events.len(), 1);
    assert_eq!(env["local"].introspect().connect_events[0].is_server, true);
    assert_eq!(env["local"].introspect().connect_events[0].is_client, true);
    assert_eq!(env["local"].introspect().connecting_events.len(), 0);
    assert_eq!(env["local"].introspect().disconnect_events.len(), 0);
}

#[test]
pub fn local_disconnect() {
    let mut env = TestEnvironment::default();

    // Stop local server and assert that disconnect event sends.
    env.create_local("local");
    env.flush_network();
    env["local"].introspect().clear();
    env["local"].network().stop();
    env.flush_network();
    assert_eq!(env["local"].introspect().connect_events.len(), 0);
    assert_eq!(env["local"].introspect().connecting_events.len(), 0);
    assert_eq!(env["local"].introspect().disconnect_events.len(), 1);
    assert_eq!(
        env["local"].introspect().disconnect_events[0].failed_to_connect,
        false
    );
}

#[test]
pub fn server_client_connect() {
    let mut env = TestEnvironment::default();

    // Create a server-client and assert that connect event sends.
    // Server-client does not send NetworkConnectingEvent
    env.create_server_client("server");
    env.flush_network();
    assert_eq!(env["server"].introspect().connect_events.len(), 1);
    assert_eq!(env["server"].introspect().connect_events[0].is_server, true);
    assert_eq!(env["server"].introspect().connect_events[0].is_client, true);
    assert_eq!(env["server"].introspect().connecting_events.len(), 0);
    assert_eq!(env["server"].introspect().disconnect_events.len(), 0);
}

#[test]
pub fn server_client_disconnect() {
    let mut env = TestEnvironment::default();

    // Stop server-client and assert that disconnect event sends.
    env.create_server_client("server");
    env.flush_network();
    env["server"].introspect().clear();
    env["server"].network().stop();
    env.flush_network();
    assert_eq!(env["server"].introspect().connect_events.len(), 0);
    assert_eq!(env["server"].introspect().connecting_events.len(), 0);
    assert_eq!(env["server"].introspect().disconnect_events.len(), 1);
    assert_eq!(
        env["server"].introspect().disconnect_events[0].failed_to_connect,
        false
    );
}

#[test]
pub fn server_connect() {
    let mut env = TestEnvironment::default();

    // Create a server and assert that connect event sends.
    // Server-client does not send NetworkConnectingEvent
    env.create_server("server");
    env.flush_network();
    assert_eq!(env["server"].introspect().connect_events.len(), 1);
    assert_eq!(env["server"].introspect().connect_events[0].is_server, true);
    assert_eq!(
        env["server"].introspect().connect_events[0].is_client,
        false
    );
    assert_eq!(env["server"].introspect().connecting_events.len(), 0);
    assert_eq!(env["server"].introspect().disconnect_events.len(), 0);
}

#[test]
pub fn server_disconnect() {
    let mut env = TestEnvironment::default();

    // Stop server and assert that disconnect event sends.
    env.create_server("server");
    env.flush_network();
    env["server"].introspect().clear();
    env["server"].network().stop();
    env.flush_network();
    assert_eq!(env["server"].introspect().connect_events.len(), 0);
    assert_eq!(env["server"].introspect().connecting_events.len(), 0);
    assert_eq!(env["server"].introspect().disconnect_events.len(), 1);
    assert_eq!(
        env["server"].introspect().disconnect_events[0].failed_to_connect,
        false
    );
}

#[test]
pub fn client_connecting() {
    let mut env = TestEnvironment::default();

    // Create client (pending connection) and check that connecting event is sent.
    env.create_server("server");
    env.create_client_pending("client", "server");
    env.flush_network();
    assert_eq!(env["client"].introspect().connect_events.len(), 0);
    assert_eq!(env["client"].introspect().connecting_events.len(), 1);
    assert_eq!(env["client"].introspect().disconnect_events.len(), 0);
    env["client"].introspect().clear();
}

#[test]
pub fn client_connect() {
    let mut env = TestEnvironment::default();

    // Accept client and check that connect event is sent.
    env.create_server("server");
    let acceptor = env.create_client_pending("client", "server");
    env.flush_network();
    acceptor.success();
    env["client"].introspect().clear();
    env.flush_network();
    assert_eq!(env["client"].introspect().connect_events.len(), 1);
    assert_eq!(
        env["client"].introspect().connect_events[0].is_server,
        false
    );
    assert_eq!(env["client"].introspect().connect_events[0].is_client, true);
    assert_eq!(env["client"].introspect().connecting_events.len(), 0);
    assert_eq!(env["server"].introspect().disconnect_events.len(), 0);
}

#[test]
pub fn client_stop() {
    let mut env = TestEnvironment::default();

    // Have client disconnect using stop() and check that disconnect event is sent.
    env.create_server("server");
    env.create_client("client", "server");
    env.flush_network();
    env["client"].introspect().clear();
    env["client"].network().stop();
    env.flush_network();
    assert_eq!(env["client"].introspect().connect_events.len(), 0);
    assert_eq!(env["client"].introspect().connecting_events.len(), 0);
    assert_eq!(env["client"].introspect().disconnect_events.len(), 1);
    assert_eq!(
        env["client"].introspect().disconnect_events[0].failed_to_connect,
        false
    );
}

#[test]
pub fn client_disconnect() {
    let mut env = TestEnvironment::default();

    // Have client disconnect when server uses stop() and check that disconnect event is sent.
    env.create_server("server");
    env.create_client("client", "server");
    env.flush_network();
    env["client"].introspect().clear();
    env["server"].network().stop();
    env.flush_network();
    assert_eq!(env["client"].introspect().connect_events.len(), 0);
    assert_eq!(env["client"].introspect().connecting_events.len(), 0);
    assert_eq!(env["client"].introspect().disconnect_events.len(), 1);
    assert_eq!(
        env["client"].introspect().disconnect_events[0].failed_to_connect,
        false
    );
}

#[test]
pub fn client_failed_to_connect() {
    let mut env = TestEnvironment::default();

    // Accept client and check that connect event is sent.
    env.create_server("server");
    let acceptor = env.create_client_pending("client", "server");
    env.flush_network();
    acceptor.fail();
    env["client"].introspect().clear();
    env.flush_network();
    assert_eq!(env["client"].introspect().connect_events.len(), 0);
    assert_eq!(env["client"].introspect().connecting_events.len(), 0);
    assert_eq!(env["client"].introspect().disconnect_events.len(), 1);
    assert_eq!(
        env["client"].introspect().disconnect_events[0].failed_to_connect,
        true
    );
}
