use super::common::prelude::*;

// Check the is_ functions in network
// These tests should be pretty self explanatory.

#[test]
pub fn disconnected() {
    let mut env = TestEnvironment::default();
    env.create_app("app");
    assert_eq!(env["app"].network().is_server(), false);
    assert_eq!(env["app"].network().is_client(), false);
    assert_eq!(env["app"].network().is_connecting(), false);
    assert_eq!(env["app"].network().is_connected(), false);
    assert_eq!(env["app"].network().is_disconnected(), true);
}

#[test]
pub fn local() {
    let mut env = TestEnvironment::default();
    env.create_local("local");
    assert_eq!(env["local"].network().is_server(), true);
    assert_eq!(env["local"].network().is_client(), true);
    assert_eq!(env["local"].network().is_connecting(), false);
    assert_eq!(env["local"].network().is_connected(), true);
    assert_eq!(env["local"].network().is_disconnected(), false);
}

#[test]
pub fn server_client() {
    let mut env = TestEnvironment::default();
    env.create_server_client("server");
    assert_eq!(env["server"].network().is_server(), true);
    assert_eq!(env["server"].network().is_client(), true);
    assert_eq!(env["server"].network().is_connecting(), false);
    assert_eq!(env["server"].network().is_connected(), true);
    assert_eq!(env["server"].network().is_disconnected(), false);
}

#[test]
pub fn server() {
    let mut env = TestEnvironment::default();
    env.create_server("server");
    assert_eq!(env["server"].network().is_server(), true);
    assert_eq!(env["server"].network().is_client(), false);
    assert_eq!(env["server"].network().is_connecting(), false);
    assert_eq!(env["server"].network().is_connected(), true);
    assert_eq!(env["server"].network().is_disconnected(), false);
}

#[test]
pub fn client_connecting() {
    let mut env = TestEnvironment::default();
    env.create_server("server");
    env.create_client_pending("client", "server");
    env.flush_network();
    assert_eq!(env["client"].network().is_server(), false);
    assert_eq!(env["client"].network().is_client(), false);
    assert_eq!(env["client"].network().is_connecting(), true);
    assert_eq!(env["client"].network().is_connected(), false);
    assert_eq!(env["client"].network().is_disconnected(), false);
}

#[test]
pub fn client_connected() {
    let mut env = TestEnvironment::default();
    env.create_server("server");
    env.create_client("client", "server");
    env.flush_network();
    assert_eq!(env["client"].network().is_server(), false);
    assert_eq!(env["client"].network().is_client(), true);
    assert_eq!(env["client"].network().is_connecting(), false);
    assert_eq!(env["client"].network().is_connected(), true);
    assert_eq!(env["client"].network().is_disconnected(), false);
}
