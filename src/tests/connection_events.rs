use super::common::prelude::*;
use bevy::prelude::*;

#[test]
pub fn local() {
    let mut app = App::new();
    app.setup_for_tests();
    app.network_mut().start_local();
    flush_network(vec![&mut app]);
    assert_eq!(app.introspect().connect_events.len(), 1);
    assert_eq!(app.introspect().disconnect_events.len(), 0);
    app.network_mut().stop();
    flush_network(vec![&mut app]);
    assert_eq!(app.introspect().connect_events.len(), 1);
    assert_eq!(app.introspect().disconnect_events.len(), 1);
}
