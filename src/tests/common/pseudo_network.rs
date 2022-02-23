use bevy_nety_protocol::{
    NetworkConnectStatus, NetworkConnectorProtocol, NetworkHostProtocol, NetworkSocket,
    NetworkSocketProtocol,
};
use std::collections::HashMap;
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Mutex,
};

// TODO: should rewrite PseudoHost and PseudoSocket so that update() is required to work
//       to ensure that network code calls it

pub struct PseudoNetwork {
    hosts: HashMap<String, Sender<PseudoHostConnection>>,
}

impl PseudoNetwork {
    pub fn new() -> Self {
        Self {
            hosts: HashMap::new(),
        }
    }

    pub fn create_host(&mut self) -> Box<PseudoHost> {
        self.create_host_named("")
    }

    pub fn create_host_named(&mut self, name: &str) -> Box<PseudoHost> {
        if !self.hosts.contains_key(name) {
            let (sender, receiver) = channel::<PseudoHostConnection>();
            self.hosts.insert(name.into(), sender);
            Box::new(PseudoHost {
                connection_receiver: Mutex::new(receiver),
            })
        } else {
            panic!("Adding two pseudo hosts with the same name");
        }
    }

    pub fn create_connector(&mut self) -> PseudoConnectorController {
        self.create_connector_named("")
    }

    pub fn create_connector_named(&mut self, host: &str) -> PseudoConnectorController {
        if let Some(sender) = self.hosts.get(host) {
            PseudoConnectorController(PseudoConnector {
                sender: Mutex::new(sender.clone()),
                success: false,
                fail: false,
                success_receiver: None,
            })
        } else {
            panic!("Adding connector to non-existent host");
        }
    }
}

#[derive(Debug)]
pub struct PseudoHostConnection {
    sender: Mutex<Sender<PseudoSocketMessage>>,
    receiver: Mutex<Receiver<PseudoSocketMessage>>,
}

pub struct PseudoHost {
    connection_receiver: Mutex<Receiver<PseudoHostConnection>>,
}

impl NetworkHostProtocol for PseudoHost {
    fn update(&mut self) {}

    fn accept(&mut self) -> Option<NetworkSocket> {
        if let Ok(connection) = self.connection_receiver.lock().unwrap().try_recv() {
            Some(Box::new(PseudoSocket {
                sender: connection.sender,
                receiver: connection.receiver,
                connected: true,
            }))
        } else {
            None
        }
    }
}

pub struct PseudoConnectorController(PseudoConnector);

impl PseudoConnectorController {
    pub fn as_success(mut self) -> Box<PseudoConnector> {
        self.0.success = true;
        Box::new(self.0)
    }

    pub fn as_fail(mut self) -> Box<PseudoConnector> {
        self.0.fail = true;
        Box::new(self.0)
    }

    pub fn as_pending(mut self) -> (Box<PseudoConnector>, PseudoConnectorAccepter) {
        let (sender, receiver) = channel::<bool>();
        self.0.success_receiver = Some(Mutex::new(receiver));
        (Box::new(self.0), PseudoConnectorAccepter { sender })
    }
}

pub struct PseudoConnectorAccepter {
    sender: Sender<bool>,
}

impl PseudoConnectorAccepter {
    pub fn success(self) {
        self.sender.send(true).unwrap();
    }

    pub fn fail(self) {
        self.sender.send(false).unwrap();
    }
}

pub struct PseudoConnector {
    sender: Mutex<Sender<PseudoHostConnection>>,
    success: bool,
    fail: bool,
    success_receiver: Option<Mutex<Receiver<bool>>>,
}

impl NetworkConnectorProtocol for PseudoConnector {
    fn status(&mut self) -> NetworkConnectStatus {
        if let Some(success_receiver) = &self.success_receiver {
            if let Ok(success) = success_receiver.lock().unwrap().try_recv() {
                if success {
                    self.success = true;
                } else {
                    self.fail = true;
                }
            }
        }
        if self.success {
            let (host_sender, socket_receiver) = channel::<PseudoSocketMessage>();
            let (socket_sender, host_receiver) = channel::<PseudoSocketMessage>();
            self.sender
                .lock()
                .unwrap()
                .send(PseudoHostConnection {
                    sender: Mutex::new(host_sender),
                    receiver: Mutex::new(host_receiver),
                })
                .unwrap();
            NetworkConnectStatus::Connected(Box::new(PseudoSocket {
                sender: Mutex::new(socket_sender),
                receiver: Mutex::new(socket_receiver),
                connected: true,
            }))
        } else if self.fail {
            NetworkConnectStatus::Failed
        } else {
            NetworkConnectStatus::Connecting
        }
    }
}

pub enum PseudoSocketMessage {
    Message(String),
    Disconnect,
}

pub struct PseudoSocket {
    sender: Mutex<Sender<PseudoSocketMessage>>,
    receiver: Mutex<Receiver<PseudoSocketMessage>>,
    connected: bool,
}

impl NetworkSocketProtocol for PseudoSocket {
    fn update(&mut self) {}

    fn connected(&mut self) -> bool {
        self.connected
    }

    fn send(&mut self, message: String) {
        if self.connected {
            self.sender
                .lock()
                .unwrap()
                .send(PseudoSocketMessage::Message(message))
                .unwrap();
        }
    }

    fn receive(&mut self) -> Option<String> {
        if self.connected {
            if let Ok(message) = self.receiver.lock().unwrap().try_recv() {
                match message {
                    PseudoSocketMessage::Message(message) => Some(message),
                    PseudoSocketMessage::Disconnect => {
                        self.connected = false;
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    fn disconnect(&mut self) {
        if self.connected {
            let _ = self
                .sender
                .lock()
                .unwrap()
                .send(PseudoSocketMessage::Disconnect);
            self.connected = false;
        }
    }
}

impl Drop for PseudoSocket {
    fn drop(&mut self) {
        self.disconnect();
    }
}

#[test]
fn pseudo_network_connect() {
    {
        let mut pseudo_net = PseudoNetwork::new();
        let _host = pseudo_net.create_host();
        let (mut connector, acceptor) = pseudo_net.create_connector().as_pending();
        assert!(matches!(
            connector.status(),
            NetworkConnectStatus::Connecting
        ));
        acceptor.success();
        assert!(matches!(
            connector.status(),
            NetworkConnectStatus::Connected(..)
        ));
    }
    {
        let mut pseudo_net = PseudoNetwork::new();
        let _host = pseudo_net.create_host();
        let mut connector = pseudo_net.create_connector().as_success();
        assert!(matches!(
            connector.status(),
            NetworkConnectStatus::Connected(..)
        ));
    }
}

#[test]
fn pseudo_network_fail_to_connect() {
    {
        let mut pseudo_net = PseudoNetwork::new();
        let _host = pseudo_net.create_host();
        let (mut connector, acceptor) = pseudo_net.create_connector().as_pending();
        assert!(matches!(
            connector.status(),
            NetworkConnectStatus::Connecting
        ));
        acceptor.fail();
        assert!(matches!(connector.status(), NetworkConnectStatus::Failed));
    }
    {
        let mut pseudo_net = PseudoNetwork::new();
        let _host = pseudo_net.create_host();
        let mut connector = pseudo_net.create_connector().as_fail();
        assert!(matches!(connector.status(), NetworkConnectStatus::Failed));
    }
}

#[test]
fn pseudo_network_send_receive() {
    let mut pseudo_net = PseudoNetwork::new();
    let mut host = pseudo_net.create_host();
    let mut connector = pseudo_net.create_connector().as_success();
    let mut connector_socket = if let NetworkConnectStatus::Connected(socket) = connector.status() {
        socket
    } else {
        panic!("Failed to connect");
    };
    let mut connection = host.accept().unwrap();
    connection.send("ping".into());
    connector_socket.send("pong".into());
    assert_eq!(connection.receive().unwrap(), "pong");
    assert_eq!(connector_socket.receive().unwrap(), "ping");
}

#[test]
fn pseudo_network_disconnect() {
    {
        let mut pseudo_net = PseudoNetwork::new();
        let mut host = pseudo_net.create_host();
        let mut connector = pseudo_net.create_connector().as_success();
        let mut connector_socket =
            if let NetworkConnectStatus::Connected(socket) = connector.status() {
                socket
            } else {
                panic!("Failed to connect");
            };
        let mut connection = host.accept().unwrap();
        connection.disconnect();
        assert!(!connection.connected());
        connector_socket.receive();
        assert!(!connector_socket.connected());
    }
    {
        let mut pseudo_net = PseudoNetwork::new();
        let mut host = pseudo_net.create_host();
        let mut connector = pseudo_net.create_connector().as_success();
        let mut connector_socket =
            if let NetworkConnectStatus::Connected(socket) = connector.status() {
                socket
            } else {
                panic!("Failed to connect");
            };
        let mut connection = host.accept().unwrap();
        connector_socket.disconnect();
        assert!(!connector_socket.connected());
        connection.receive();
        assert!(!connection.connected());
    }
}

#[test]
fn pseudo_network_drop_disconnect() {
    {
        let mut pseudo_net = PseudoNetwork::new();
        let mut host = pseudo_net.create_host();
        let mut connector = pseudo_net.create_connector().as_success();
        let mut connector_socket =
            if let NetworkConnectStatus::Connected(socket) = connector.status() {
                socket
            } else {
                panic!("Failed to connect");
            };
        let mut connection = Some(host.accept().unwrap());
        connection.take();
        connector_socket.receive();
        assert!(!connector_socket.connected());
    }
    {
        let mut pseudo_net = PseudoNetwork::new();
        let mut host = pseudo_net.create_host();
        let mut connector = pseudo_net.create_connector().as_success();
        let mut connector_socket =
            if let NetworkConnectStatus::Connected(socket) = connector.status() {
                Some(socket)
            } else {
                panic!("Failed to connect");
            };
        let mut connection = host.accept().unwrap();
        connector_socket.take();
        connection.receive();
        assert!(!connection.connected());
    }
}
