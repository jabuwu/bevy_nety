use super::player::NetworkPlayer;

#[derive(Clone, Debug)]
pub struct NetworkConnectEvent {
    pub is_server: bool,
    pub is_client: bool,
}

#[derive(Clone, Debug)]
pub struct NetworkConnectingEvent;

#[derive(Clone, Debug)]
pub struct NetworkDisconnectEvent {
    pub failed_to_connect: bool,
}

#[derive(Debug, Clone)]
pub struct NetworkPlayerJoinEvent {
    pub player: NetworkPlayer,
    pub me: bool,
    pub existing_player: bool,
}

#[derive(Debug, Clone)]
pub struct NetworkPlayerLeaveEvent {
    pub player: NetworkPlayer,
}

#[derive(Clone, Debug)]
pub struct NetworkEvent;
