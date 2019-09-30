pub mod types;
pub mod receiver;
pub mod sender;

pub use types::NetworkEvent;
pub use receiver::Receiver;
pub use sender::Sender;

#[cfg(test)]
mod test {
    use super::types;
    use super::{
        Receiver,
        Sender,
    };
    use std::net::SocketAddr;
    use std::time::Duration;

    #[test]
    fn player_join_event() {
        let receiver_socket: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let event_receiver = Receiver::new(receiver_socket).unwrap();
        let receiver_address = event_receiver.socket.local_addr().unwrap();

        let sender_socket: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let event_sender = Sender::new(sender_socket, receiver_address).unwrap();
        let sender_address = event_sender.socket.local_addr().unwrap();

        event_sender.register_self(receiver_address.port()).unwrap();
        let actual_data = event_receiver.peek_event(Duration::from_millis(1000)).unwrap();

        let expected_data = types::NetworkData {
            amt: 6,
            src: sender_address,
            event: types::NetworkEvent::PlayerJoin(receiver_address.port()),
        };

        assert_eq!(actual_data, expected_data);
    }
}
