use bevy_nety_protocol::{NetworkHostProtocol, NetworkSocket, NetworkSocketProtocol};
use std::sync::{
    mpsc::{channel, Receiver, Sender, TryRecvError},
    Mutex,
};

// TODO: unit tests?

pub(crate) struct InternalHost {
    socket: Option<InternalSocket>,
}

impl InternalHost {
    pub(crate) fn new_pair() -> (Box<InternalHost>, Box<InternalSocket>) {
        let (host_sender, client_receiver) = channel::<String>();
        let (client_sender, host_receiver) = channel::<String>();
        (
            Box::new(InternalHost {
                socket: Some(InternalSocket {
                    sender: Mutex::new(host_sender),
                    receiver: Mutex::new(host_receiver),
                }),
            }),
            Box::new(InternalSocket {
                sender: Mutex::new(client_sender),
                receiver: Mutex::new(client_receiver),
            }),
        )
    }
}

impl NetworkHostProtocol for InternalHost {
    fn update(&mut self) {}
    fn accept(&mut self) -> Option<NetworkSocket> {
        if let Some(socket) = self.socket.take() {
            Some(Box::new(socket) as NetworkSocket)
        } else {
            None
        }
    }
}

pub(crate) struct InternalSocket {
    sender: Mutex<Sender<String>>,
    receiver: Mutex<Receiver<String>>,
}

impl NetworkSocketProtocol for InternalSocket {
    fn update(&mut self) {}
    fn connected(&mut self) -> bool {
        true
    }
    fn send(&mut self, message: String) {
        match self.sender.lock() {
            Ok(sender) => {
                if let Err(e) = sender.send(message) {
                    panic!("{}", e);
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
    fn receive(&mut self) -> Option<String> {
        match self.receiver.lock() {
            Ok(receiver) => match receiver.try_recv() {
                Ok(message) => Some(message),
                Err(e) if e != TryRecvError::Empty => {
                    panic!("{}", e);
                }
                Err(_) => None,
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
    fn disconnect(&mut self) {}
}
