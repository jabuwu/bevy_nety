pub type NetworkHost = Box<dyn NetworkHostProtocol + Send + Sync>;
pub type NetworkSocket = Box<dyn NetworkSocketProtocol + Send + Sync>;

// TODO: these traits should deal with u8 array messages, not strings

pub trait NetworkHostProtocol {
    fn update(&mut self);
    fn accept(&mut self) -> Option<NetworkSocket>;
}

pub trait NetworkSocketProtocol {
    fn update(&mut self);
    fn connected(&mut self) -> bool;
    fn send(&mut self, message: String);
    fn receive(&mut self) -> Option<String>;
    fn disconnect(&mut self);
}
