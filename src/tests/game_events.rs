use super::common::prelude::*;
use bevy::prelude::*;

#[test]
pub fn local() {
    let mut app = App::new();
    app.setup_for_tests();
    app.network_mut().start_local();
    flush_network(vec![&mut app]);
    app.network_mut().server_mut().unwrap().send_event();
    flush_network(vec![&mut app]);
    assert_eq!(app.introspect().network_events.len(), 1);
}
