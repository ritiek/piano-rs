use serde_derive::{Serialize, Deserialize};
use std::net::SocketAddr;
pub use crate::game::{Note, GameEvent};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum NetworkEvent {
    ID(usize),
    Note(Note),
    Peers(u16, Vec<SocketAddr>),
    PlayerJoin(u16),
    PlayerLeft(u16),
}

#[derive(Debug, PartialEq)]
pub struct NetworkData {
    pub amt: usize,
    pub src: SocketAddr,
    pub event: NetworkEvent,
}

