mod add_network_data;
mod client;
mod entity;
mod event_queue;
mod events;
mod internal_protocol;
mod messages;
mod network;
mod network_type_name;
mod player;
mod player_data;
mod plugin;
mod registry;
mod relevancy;
mod serialized_struct;
mod serializer;
mod server;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::{
        add_network_data::AddNetworkData,
        client::NetworkClient,
        entity::NetworkEntity,
        events::{
            NetworkConnectEvent, NetworkConnectingEvent, NetworkDisconnectEvent, NetworkEvent,
            NetworkPlayerJoinEvent, NetworkPlayerLeaveEvent, NetworkServerEvent,
        },
        network::Network,
        player::NetworkPlayer,
        plugin::NetworkPlugin,
        server::NetworkServer,
    };
}
