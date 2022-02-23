use super::common::prelude::*;
use bevy::prelude::*;

#[test]
pub fn local() {
    let mut app = App::new();
    app.setup_for_tests();
    assert_eq!(app.network_mut().is_server(), false);
    assert_eq!(app.network_mut().is_client(), false);
    assert_eq!(app.network_mut().is_connected(), false);
    assert_eq!(app.network_mut().is_disconnected(), true);
    app.network_mut().start_local();
    assert_eq!(app.network_mut().is_server(), true);
    assert_eq!(app.network_mut().is_client(), true);
    assert_eq!(app.network_mut().is_connected(), true);
    assert_eq!(app.network_mut().is_disconnected(), false);
}
