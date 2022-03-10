use super::common::prelude::*;

// Test a few edge cases when lots of clients join/leave at once

const COUNT: usize = 10;

#[test]
fn joins_multiple() {
    let mut env = TestEnvironment::default();

    env.create_server("server");
    for i in 0..COUNT {
        env.create_client(&format!("client{}", i), "server");
    }
    env.flush_network();

    for i in 0..COUNT {
        assert_eq!(
            env[&format!("client{}", i)]
                .introspect()
                .player_join_events
                .len(),
            COUNT
        );
        assert_eq!(
            env[&format!("client{}", i)].network().players().len(),
            COUNT
        );
    }
}

#[test]
fn leaves_multiple() {
    let mut env = TestEnvironment::default();

    env.create_server("server");
    for i in 0..COUNT {
        env.create_client(&format!("client{}", i), "server");
    }
    env.flush_network();

    for i in 0..COUNT {
        if i != 0 {
            env[&format!("client{}", i)].network().stop();
        }
    }
    env.flush_network();

    for i in 0..COUNT {
        if i != 0 {
            assert_eq!(env[&format!("client{}", i)].network().players().len(), 0);
        }
    }

    assert_eq!(env["client0"].network().players().len(), 1);
    assert_eq!(
        env["client0"].introspect().player_leave_events.len(),
        COUNT - 1
    );
    assert_eq!(env["server"].network().players().len(), 1);
    assert_eq!(
        env["server"].introspect().player_leave_events.len(),
        COUNT - 1
    );
}
