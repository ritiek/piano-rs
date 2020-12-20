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
            socket,
            host_addr,
            peer_addrs: Vec::new(),
        })
    }

    pub fn register_self(&self, receiver_port: u16) -> Result<()> {
        let bytes = bincode::serialize(&types::NetworkEvent::PlayerJoin(receiver_port)).unwrap();
        self.socket.send_to(&bytes, self.host_addr)?;
        Ok(())
    }

    pub fn register_remote_socket(&mut self, receiver_port: u16, client_addr: SocketAddr) -> Result<()> {
        if !self.peer_addrs.contains(&client_addr) {
            self.peer_addrs.push(client_addr);
        }

        let id = self.peer_addrs
            .iter()
            .position(|&peer_addr| peer_addr == client_addr).unwrap();
        let id_bytes = bincode::serialize(&types::NetworkEvent::ID(id)).unwrap();
        self.socket.send_to(&id_bytes, client_addr)?;

        let peer_addrs_clone = self.peer_addrs.clone();
        let peer_addrs_bytes = bincode::serialize(&types::NetworkEvent::Peers(receiver_port, peer_addrs_clone)).unwrap();
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

