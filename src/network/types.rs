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

