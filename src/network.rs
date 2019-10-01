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
    fn register_self() {
        let receiver_socket: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let event_receiver = Receiver::new(receiver_socket).unwrap();
        let receiver_address = event_receiver.socket.local_addr().unwrap();

        let sender_socket: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let event_sender = Sender::new(sender_socket, receiver_address).unwrap();
        let sender_address = event_sender.socket.local_addr().unwrap();

        event_sender.register_self(receiver_address.port()).unwrap();
        let actual_registration_data = event_receiver.peek_event(Duration::from_millis(1000)).unwrap();

        let expected_registration_data = types::NetworkData {
            amt: 6,
            src: sender_address,
            event: types::NetworkEvent::PlayerJoin(receiver_address.port()),
        };

        assert_eq!(actual_registration_data, expected_registration_data);
    }

    #[test]
    fn register_remote_socket() {
        let receiver_socket: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let event_receiver = Receiver::new(receiver_socket).unwrap();
        let receiver_address = event_receiver.socket.local_addr().unwrap();

        let sender_socket: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let mut event_sender = Sender::new(sender_socket, receiver_address).unwrap();
        let sender_address = event_sender.socket.local_addr().unwrap();

        let client_socket: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let client_event = Receiver::new(client_socket).unwrap();
        let client_address = client_event.socket.local_addr().unwrap();

        event_sender.register_remote_socket(receiver_address.port(), client_address).unwrap();

        let actual_client_id = client_event.peek_event(Duration::from_millis(1000)).unwrap();
        let expected_client_id = types::NetworkData {
            amt: 12,
            src: sender_address,
            event: types::NetworkEvent::ID(0),
        };

        assert_eq!(actual_client_id, expected_client_id);

        let actual_peers_received_by_client = client_event.peek_event(Duration::from_millis(1000)).unwrap();
        let expected_peers_received_by_client = types::NetworkData {
            amt: 24,
            src: sender_address,
            event: types::NetworkEvent::Peers(receiver_address.port(), vec![client_address])
        };

        assert_eq!(actual_peers_received_by_client, expected_peers_received_by_client);
    }

    #[test]
    fn register_self_and_register_remote_socket_combined() {
        let receiver_socket: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let event_receiver = Receiver::new(receiver_socket).unwrap();
        let receiver_address = event_receiver.socket.local_addr().unwrap();

        let sender_socket: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let mut event_sender = Sender::new(sender_socket, receiver_address).unwrap();
        let sender_address = event_sender.socket.local_addr().unwrap();

        let client_socket: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let client_event = Receiver::new(client_socket).unwrap();
        let client_address = client_event.socket.local_addr().unwrap();

        event_sender.register_self(receiver_address.port()).unwrap();
        let actual_registration_data = event_receiver.peek_event(Duration::from_millis(1000)).unwrap();
        let expected_registration_data = types::NetworkData {
            amt: 6,
            src: sender_address,
            event: types::NetworkEvent::PlayerJoin(receiver_address.port()),
        };

        assert_eq!(actual_registration_data, expected_registration_data);

        event_sender.register_remote_socket(receiver_address.port(), client_address).unwrap();
        let actual_client_id = client_event.peek_event(Duration::from_millis(1000)).unwrap();
        let actual_peers_received_by_client = client_event.peek_event(Duration::from_millis(1000)).unwrap();

        let expected_client_id = types::NetworkData {
            amt: 12,
            src: sender_address,
            event: types::NetworkEvent::ID(0),
        };
        let expected_peers_received_by_client = types::NetworkData {
            amt: 24,
            src: sender_address,
            event: types::NetworkEvent::Peers(receiver_address.port(), vec![client_address])
        };

        assert_eq!(actual_client_id, expected_client_id);
        assert_eq!(actual_peers_received_by_client, expected_peers_received_by_client);
    }
}
