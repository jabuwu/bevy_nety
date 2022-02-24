use super::common::prelude::*;
use bevy::prelude::*;

// Check a few edge cases when lots of clients join/leave at once

const COUNT: usize = 10;

#[test]
fn joins_multiple() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client_apps = vec![];
    for _ in 0..COUNT {
        client_apps.push(App::new());
    }
    server_app.setup_for_tests();
    for app in client_apps.iter_mut() {
        app.setup_for_tests();
    }
    server_app
        .network_mut()
        .start_server(vec![pseudo_net.create_host()]);
    for app in client_apps.iter_mut() {
        app.network_mut()
            .start_client(pseudo_net.create_connector().as_success());
    }
    flush_network_2(vec![&mut server_app], &mut client_apps);
    for app in client_apps.iter_mut() {
        assert_eq!(app.introspect().player_join_events.len(), COUNT);
        assert_eq!(app.network().players().len(), COUNT);
    }
}

#[test]
fn leaves_multiple() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut server_app = App::new();
    let mut client_apps = vec![];
    for _ in 0..COUNT {
        client_apps.push(App::new());
    }
    server_app.setup_for_tests();
    for app in client_apps.iter_mut() {
        app.setup_for_tests();
    }
    server_app
        .network_mut()
        .start_server(vec![pseudo_net.create_host()]);
    for app in client_apps.iter_mut() {
        app.network_mut()
            .start_client(pseudo_net.create_connector().as_success());
    }
    flush_network_2(vec![&mut server_app], &mut client_apps);
    for (index, app) in client_apps.iter_mut().enumerate() {
        if index != 0 {
            app.network_mut().stop();
        }
    }
    flush_network_2(vec![&mut server_app], &mut client_apps);
    for (index, app) in client_apps.iter_mut().enumerate() {
        if index != 0 {
            assert_eq!(app.network().players().len(), 0);
        }
    }
    assert_eq!(client_apps[0].network().players().len(), 1);
    assert_eq!(
        client_apps[0].introspect().player_leave_events.len(),
        COUNT - 1
    );
    assert_eq!(server_app.network().players().len(), 1);
    assert_eq!(server_app.introspect().player_leave_events.len(), COUNT - 1);
}
