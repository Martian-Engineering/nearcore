use near_primitives::network::PeerId;
use std::sync::Arc;
use near_o11y::tracing::info;
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, BorshSerialize, BorshDeserialize)]
struct SerializableString(String);

impl From<String> for SerializableString {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<SerializableString> for String {
    fn from(s: SerializableString) -> Self {
        s.0
    }
}


/// Enum containing all possible message types that can be sent on the network
#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq, Hash, Clone)]
pub struct PorMessage {
    pub content: String,
}

/// PoR protocol handler.
/// This implementation is a simple echo server - when it receives a string message from a peer,
/// it sends the same string back as a PoR response.
pub struct PorHandler {
    my_peer_id: PeerId,
    send_message: Arc<dyn Fn(&PeerId, PorMessage) + Send + Sync>,
}

impl PorHandler {
    pub fn new(
        my_peer_id: PeerId,
        send_message: impl Fn(&PeerId, PorMessage) + Send + Sync + 'static,
    ) -> Self {
        Self {
            my_peer_id,
            send_message: Arc::new(send_message),
        }
    }

    /// Sends a PoR message to the specified peer
    pub fn send_message(&self, target: &PeerId, content: PorMessage) {
        info!(target: "por", "Sending PoR message to {}: {:?}", target, content);
        (self.send_message)(target, content);
    }

    /// Handles a received PoR message
    pub fn handle_message(&self, source: &PeerId, message: PorMessage) {
        info!(target: "por", "Received message from {}: {:?}", source, message);
        // Echo back the received message as a response
        self.send_message(source, PorMessage { content: message.content });
    }
}