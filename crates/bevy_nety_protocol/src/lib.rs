pub type NetworkConnector = Box<dyn NetworkConnectorProtocol + Send + Sync>;
pub type NetworkHost = Box<dyn NetworkHostProtocol + Send + Sync>;
pub type NetworkSocket = Box<dyn NetworkSocketProtocol + Send + Sync>;

// any implementation of these traits must be reliable and ordered
// TODO: allow sending unreliably (for some messages)
//       a UDP implementation will need a reliable option

// TODO: these traits should deal with u8 array messages, not strings

pub trait NetworkHostProtocol {
    fn update(&mut self);
    fn accept(&mut self) -> Option<NetworkSocket>;
}

pub enum NetworkConnectStatus {
    Connected(NetworkSocket),
    Connecting,
    Failed,
}

pub trait NetworkConnectorProtocol {
    fn status(&mut self) -> NetworkConnectStatus;
}

pub trait NetworkSocketProtocol {
    fn update(&mut self);
    fn connected(&mut self) -> bool;
    fn send(&mut self, message: String);
    fn receive(&mut self) -> Option<String>;
    fn disconnect(&mut self);
}
