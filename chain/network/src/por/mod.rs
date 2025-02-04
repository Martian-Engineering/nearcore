use near_primitives::network::PeerId;
use std::sync::Arc;
use near_o11y::tracing::info;
use borsh::{BorshSerialize, BorshDeserialize};

use chrono::{DateTime, Utc};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Hash, BorshSerialize, BorshDeserialize)]
pub struct RequestId {
    pub node: String,
    pub timestamp: DateTime<Utc>,
    pub sequence: u32,
}

impl RequestId {
    pub fn new(node: String, sequence: u32) -> Self {
        Self {
            node,
            timestamp: Utc::now(),
            sequence,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}-{}-{:04}", self.node, self.timestamp.timestamp(), self.sequence)
    }
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct RequestMessage {
    pub request_id: RequestId,
    pub payload: String,
    pub path: Vec<String>,
    pub total_latency: Duration,
    pub total_cost: u64,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct ResponseMessage {
    pub request_id: RequestId,
    pub payload: String,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct EdgeCutMessage {
    pub request_id: RequestId,
    pub node1: String,
    pub node2: String,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct SyncMessage {
    pub request_id: RequestId,
    pub timestamp: DateTime<Utc>,
    pub next_expected: DateTime<Utc>,
    pub has_late_payment: bool,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct PaymentMessage {
    pub request_id: RequestId,
    pub amount: u64,
    pub latency_so_far: Duration,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct AckMessage {
    pub request_id: RequestId,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct NodeIdMessage {
    pub node_name: String,
    pub node_version: u32,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub enum PorMessage {
    Request(RequestMessage),
    Response(ResponseMessage), 
    EdgeCut(EdgeCutMessage),
    Sync(SyncMessage),
    Payment(PaymentMessage),
    Ack(AckMessage),
    NodeId(NodeIdMessage),
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