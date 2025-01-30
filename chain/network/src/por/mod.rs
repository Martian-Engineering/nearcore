use near_primitives::network::PeerId;
use std::sync::Arc;
use near_o11y::tracing::info;
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq, Hash, Clone)]
pub struct PorMessage {
    pub content: String,
}

/// PoR protocol handler.
/// This implementation is a simple echo server - when it receives a string message from a peer,
/// it sends the same string back as a PoR response.
pub struct PorHandler {
    my_peer_id: PeerId,
    send_message: Arc<dyn Fn(&PeerId, String) + Send + Sync>,
}

impl PorHandler {
    pub fn new(
        my_peer_id: PeerId,
        send_message: impl Fn(&PeerId, String) + Send + Sync + 'static,
    ) -> Self {
        Self {
            my_peer_id,
            send_message: Arc::new(send_message),
        }
    }

    /// Sends a PoR message to the specified peer
    pub fn send_message(&self, target: &PeerId, content: String) {
        info!(target: "por", "Sending PoR message to {}: {}", target, content);
        (self.send_message)(target, content);
    }

    /// Handles a received PoR message
    pub fn handle_message(&self, source: &PeerId, message: PorMessage) {
        info!(target: "por", "Received PoR message from {}: {}", source, message.content);
        // Echo the message back
        self.send_message(source, message.content);
    }
}