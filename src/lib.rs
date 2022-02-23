mod client;
mod event_queue;
mod events;
mod internal_protocol;
mod messages;
mod network;
mod player;
mod plugin;
mod serializer;
mod server;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::{
        client::NetworkClient,
        events::{
            NetworkConnectEvent, NetworkConnectingEvent, NetworkDisconnectEvent, NetworkEvent,
            NetworkPlayerJoinEvent, NetworkPlayerLeaveEvent,
        },
        network::Network,
        player::NetworkPlayer,
        plugin::NetworkPlugin,
        server::NetworkServer,
    };
}
