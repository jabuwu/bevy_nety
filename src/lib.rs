mod client;
mod events;
mod internal_protocol;
mod messages;
mod network;
mod plugin;
mod serializer;
mod server;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::{
        client::NetworkClient,
        events::{NetworkConnectEvent, NetworkDisconnectEvent, NetworkEvent},
        network::Network,
        plugin::NetworkPlugin,
        server::NetworkServer,
    };
}
