use bevy_nety_protocol::{
    NetworkConnectStatus, NetworkConnectorProtocol, NetworkHostProtocol, NetworkSocket,
    NetworkSocketProtocol,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, ErrorKind, Read, Result, Write};
use std::net::{Shutdown, TcpListener, TcpStream, ToSocketAddrs};

// TODO: replace with a threaded/blocking implementation
// TODO: remove panic inducing unwraps

pub struct TcpHost {
    listener: TcpListener,
}

impl TcpHost {
    pub fn listen<A: ToSocketAddrs>(addr: A) -> Result<Box<TcpHost>> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        Ok(Box::new(TcpHost { listener }))
    }
}

impl NetworkHostProtocol for TcpHost {
    fn update(&mut self) {}
    fn accept(&mut self) -> Option<NetworkSocket> {
        if let Ok((stream, _)) = self.listener.accept() {
            stream.set_nonblocking(true).unwrap();
            Some(Box::new(TcpSocket {
                stream,
                write_buffer: vec![],
                read_buffer: vec![],
                connected: true,
            }))
        } else {
            None
        }
    }
}

pub struct TcpConnector {
    socket: Option<TcpSocket>,
}

impl TcpConnector {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Box<Self> {
        if let Ok(stream) = TcpStream::connect(addr) {
            stream.set_nonblocking(true).unwrap();
            Box::new(Self {
                socket: Some(TcpSocket {
                    stream,
                    write_buffer: vec![],
                    read_buffer: vec![],
                    connected: true,
                }),
            })
        } else {
            Box::new(Self { socket: None })
        }
    }
}

impl NetworkConnectorProtocol for TcpConnector {
    fn status(&mut self) -> NetworkConnectStatus {
        if let Some(socket) = self.socket.take() {
            NetworkConnectStatus::Connected(Box::new(socket))
        } else {
            NetworkConnectStatus::Failed
        }
    }
}

pub struct TcpSocket {
    stream: TcpStream,
    write_buffer: Vec<u8>,
    read_buffer: Vec<u8>,
    connected: bool,
}

impl NetworkSocketProtocol for TcpSocket {
    fn update(&mut self) {
        let mut buf: [u8; 16384] = [0; 16384];
        match self.stream.read(&mut buf) {
            Ok(len) if len == 0 => {
                self.connected = false;
            }
            Err(ref e) if e.kind() != ErrorKind::WouldBlock => {
                self.connected = false;
            }
            Ok(len) => {
                self.read_buffer.write_all(&buf[..len]).unwrap();
            }
            _ => {}
        }
    }
    fn connected(&mut self) -> bool {
        self.connected
    }
    fn send(&mut self, message: String) {
        self.write_buffer
            .write_u16::<LittleEndian>(message.len() as u16)
            .unwrap();
        self.write_buffer.write_all(message.as_bytes()).unwrap();
        if let Ok(len) = self.stream.write(&self.write_buffer) {
            self.write_buffer.drain(0..len);
        }
    }
    fn receive(&mut self) -> Option<String> {
        if self.read_buffer.len() >= 2 {
            let len = Cursor::new([self.read_buffer[0], self.read_buffer[1]])
                .read_u16::<LittleEndian>()
                .unwrap() as usize;
            if self.read_buffer.len() >= len + 2 {
                let message = String::from_utf8(self.read_buffer[2..len + 2].to_vec()).unwrap();
                self.read_buffer.drain(0..len + 2);
                Some(message)
            } else {
                None
            }
        } else {
            None
        }
    }
    fn disconnect(&mut self) {
        let _ = self.stream.shutdown(Shutdown::Both).unwrap();
    }
}

pub mod prelude {
    pub use super::{TcpConnector, TcpHost};
}
