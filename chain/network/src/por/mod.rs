use near_primitives::network::PeerId;
use std::sync::Arc;
use near_o11y::tracing::info;
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq, Hash, Clone)]
pub struct RequestMessage {
    pub content: String,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq, Hash, Clone)]
pub struct ResponseMessage {
    pub content: String,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq, Hash, Clone)]
pub enum PorMessage {
    Request(RequestMessage),
    Response(ResponseMessage),
//    EdgeCut(EdgeCutMessage),
//    Sync(SyncMessage),
//    Payment(PaymentMessage),
//    Ack(AckMessage),
}

/// PoR protocol handler.
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
    pub fn send_message(&self, target: &PeerId, message: PorMessage) {
        match message.clone() {
            PorMessage::Request(request) => {
                info!(target: "por", "Sending PoR request to {}: {:?}", target, request);
            }
            PorMessage::Response(response) => {
                info!(target: "por", "Sending PoR response to {}: {:?}", target, response);
            }
        }
        (self.send_message)(target, message);
    }

    /// Handles a received PoR message
    pub fn handle_message(&self, source: &PeerId, message: PorMessage) {
        match message {
            PorMessage::Request(request) => {
                info!(target: "por", "Received PoR request from {}: {:?}", source, request);
                self.send_message(source, PorMessage::Response(ResponseMessage { content: request.content }));
            }
            PorMessage::Response(response) => {
                info!(target: "por", "Received PoR response from {}: {:?}", source, response);
            }
        }
    }
}