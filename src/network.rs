pub use receiver::Receiver;
pub use sender::Sender;
pub use types::NetworkEvent;

pub mod types {
    use serde_derive::{Serialize, Deserialize};
    use std::net::SocketAddr;
    pub use crate::game::{Note, GameEvent};


    #[derive(Debug, Serialize, Deserialize)]
    pub enum NetworkEvent {
        ID(usize),
        Note(Note),
        Peers(Vec<SocketAddr>),
        PlayerJoin,
        PlayerLeft,
    }

    #[derive(Debug)]
    pub struct NetworkData {
        pub amt: usize,
        pub src: SocketAddr,
        pub event: NetworkEvent,
    }
}

pub mod receiver {
    use std::time;
    use std::net::{SocketAddr, UdpSocket};
    use std::io::Result;
    use crate::network::types;

    #[derive(Debug)]
    pub struct Receiver {
        socket: UdpSocket,
    }

    impl Receiver {
        pub fn new(addr: SocketAddr) -> Result<Receiver> {
            let socket = UdpSocket::bind(&addr)?;
            Ok(Receiver {
                socket: socket,
            })
        }

        pub fn poll_event(&self) -> Result<types::NetworkData> {
            self.socket.set_read_timeout(None)?;

            let mut buf = [0; 300];
            let (amt, src) = self.socket.recv_from(&mut buf)?;

            let event: types::NetworkEvent = bincode::deserialize(&buf).unwrap();
            Ok(types::NetworkData {
                amt: amt,
                src: src,
                event: event,
            })
        }

        pub fn peek_event(&self, duration: time::Duration) -> Result<types::NetworkData> {
            self.socket.set_read_timeout(Some(duration))?;

            let mut buf = [0; 300];
            let (amt, src) = self.socket.recv_from(&mut buf)?;

            let event: types::NetworkEvent = bincode::deserialize(&buf).unwrap();
            Ok(types::NetworkData {
                amt: amt,
                src: src,
                event: event,
            })
        }
    }
}

pub mod sender {
    use std::net::{SocketAddr, UdpSocket};
    use std::io::Result;
    use crate::network::types;


    #[derive(Debug)]
    pub struct Sender {
        pub socket: UdpSocket,
        pub host_addr: SocketAddr,
        pub peer_addrs: Vec<SocketAddr>,
    }

    impl Sender {
        pub fn new(addr: SocketAddr, host_addr: SocketAddr) -> Result<Sender> {
            let socket = UdpSocket::bind(&addr)?;
            Ok(Sender {
                socket: socket,
                host_addr: host_addr,
                peer_addrs: Vec::new(),
            })
        }

        pub fn register_self(&self) -> Result<()> {
            let bytes = bincode::serialize(&types::NetworkEvent::PlayerJoin).unwrap();
            self.socket.send_to(&bytes, self.host_addr)?;
            Ok(())
        }

        pub fn register_remote_socket(&mut self, addr: SocketAddr) -> Result<()> {
            if !self.peer_addrs.contains(&addr) {
                self.peer_addrs.push(addr);
            }

            let id = self.peer_addrs
                .iter()
                .position(|&peer_addr| peer_addr == addr).unwrap();
            let id_bytes = bincode::serialize(&types::NetworkEvent::ID(id)).unwrap();
            self.socket.send_to(&id_bytes, addr)?;

            let peer_addrs_clone = self.peer_addrs.clone();
            let peer_addrs_bytes = bincode::serialize(&types::NetworkEvent::Peers(peer_addrs_clone)).unwrap();
            for peer_addr in self.peer_addrs.iter() {
                self.socket.send_to(&peer_addrs_bytes, peer_addr)?;
            }

            Ok(())
        }

        pub fn tick(&self, note: types::Note) -> Result<()> {
            let bytes = bincode::serialize(&types::NetworkEvent::Note(note)).unwrap();
            for peer_addr in self.peer_addrs.iter() {
                self.socket.send_to(&bytes, peer_addr)?;
            }
            Ok(())
        }
    }
}
