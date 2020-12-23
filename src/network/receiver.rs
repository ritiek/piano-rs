use std::time;
use std::net::{SocketAddr, UdpSocket};
use std::io::Result;
use crate::network::types;

#[derive(Debug)]
pub struct Receiver {
    pub socket: UdpSocket,
}

impl Receiver {
    pub fn new(addr: SocketAddr) -> Result<Receiver> {
        let socket = UdpSocket::bind(&addr)?;
        Ok(Receiver {
            socket,
        })
    }

    pub fn poll_event(&self) -> Result<types::NetworkData> {
        self.socket.set_read_timeout(None)?;
        let mut buf = [0; 300];
        loop {
            let (amt, src) = self.socket.recv_from(&mut buf)?;
            let result = bincode::deserialize(&buf);
            if let Ok(event) = result {
                break Ok(types::NetworkData {
                    amt,
                    src,
                    event,
                })
            }
        }
    }

    pub fn peek_event(&self, duration: time::Duration) -> Result<types::NetworkData> {
        self.socket.set_read_timeout(Some(duration))?;

        let mut buf = [0; 300];
        let (amt, src) = self.socket.recv_from(&mut buf)?;

        let event: types::NetworkEvent = bincode::deserialize(&buf).unwrap();
        Ok(types::NetworkData {
            amt,
            src,
            event,
        })
    }
}

