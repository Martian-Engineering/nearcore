use crate::network_protocol::proto;
use crate::network_protocol::proto::proof_of_response::ResponseType;
use crate::por::{
    PorMessage, RequestId, RequestMessage, ResponseMessage,
    EdgeCutMessage, SyncMessage, PaymentMessage, AckMessage, NodeIdMessage
};
use chrono::{DateTime, Utc};
use std::time::Duration;
use protobuf::MessageField as MF;
use crate::network_protocol::proto_conv::error::{ParseRequiredError, ParseTimestampError};

// Convert from protobuf Timestamp to chrono::DateTime<Utc>
fn from_proto_timestamp(ts: &google::protobuf::Timestamp) -> DateTime<Utc> {
    DateTime::<Utc>::from_timestamp(ts.seconds, ts.nanos as u32)
        .expect("invalid timestamp")
}

// Convert to protobuf Timestamp from chrono::DateTime<Utc>
fn to_proto_timestamp(dt: &DateTime<Utc>) -> google::protobuf::Timestamp {
    let mut ts = google::protobuf::Timestamp::new();
    ts.seconds = dt.timestamp();
    ts.nanos = dt.timestamp_subsec_nanos() as i32;
    ts
}

// Convert Duration to milliseconds
fn duration_to_millis(d: Duration) -> u64 {
    d.as_millis() as u64
}

// Convert milliseconds to Duration  
fn millis_to_duration(ms: u64) -> Duration {
    Duration::from_millis(ms)
}

// RequestId conversion
impl From<&RequestId> for proto::ProofOfResponse {
    fn from(x: &RequestId) -> Self {
        Self {
            node: x.node.clone(),
            timestamp: MF::some(&to_proto_timestamp(&x.timestamp)),
            sequence: x.sequence,
            response_type: None,
            ..Default::default()
        }
    }
}

impl TryFrom<&proto::ProofOfResponse> for RequestId {
    type Error = ParseProofOfResponseError;
    fn try_from(x: &proto::ProofOfResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            node: x.node.clone(),
            timestamp: from_proto_timestamp(x.timestamp.as_ref().ok_or_else(|| Self::Error::Timestamp(ParseRequiredError::new()))?),
            sequence: x.sequence,
        })
    }
}

// RequestMessage conversion
impl From<&RequestMessage> for proto::RequestMessage {
    fn from(x: &RequestMessage) -> Self {
        Self {
            payload: x.payload.clone(),
            path: x.path.clone(),
            total_latency_ms: duration_to_millis(x.total_latency),
            total_cost: x.total_cost,
            ..Default::default()
        }
    }
}

impl TryFrom<&proto::RequestMessage> for RequestMessage {
    type Error = ParseRequestMessageError;
    fn try_from(x: &proto::RequestMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            request_id: try_from_required(&proto::ProofOfResponse {
                node: x.payload.clone(),  // Temporary mapping until we have proper RequestId serialization
                timestamp: None,
                sequence: 0,
                response_type: None,
                ..Default::default()
            }).map_err(Self::Error::RequestId)?,
            payload: x.payload.clone(),
            path: x.path.clone(),
            total_latency: millis_to_duration(x.total_latency_ms),
            total_cost: x.total_cost,
        })
    }
}

// ResponseMessage conversion
impl From<&ResponseMessage> for proto::ResponseMessage {
    fn from(x: &ResponseMessage) -> Self {
        Self {
            payload: x.payload.clone(),
            ..Default::default()
        }
    }
}

impl TryFrom<&proto::ResponseMessage> for ResponseMessage {
    type Error = ParseResponseMessageError;
    fn try_from(x: &proto::ResponseMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            request_id: try_from_required(&proto::ProofOfResponse {
                node: x.payload.clone(),  // Temporary mapping until we have proper RequestId serialization
                timestamp: None,
                sequence: 0,
                response_type: None,
                ..Default::default()
            }).map_err(Self::Error::RequestId)?,
            payload: x.payload.clone(),
        })
    }
}

// EdgeCutMessage conversion  
impl From<&EdgeCutMessage> for proto::EdgeCutMessage {
    fn from(x: &EdgeCutMessage) -> Self {
        Self {
            node1: x.node1.clone(),
            node2: x.node2.clone(),
            ..Default::default()
        }
    }
}

impl TryFrom<&proto::EdgeCutMessage> for EdgeCutMessage {
    type Error = ParseEdgeCutMessageError;
    fn try_from(x: &proto::EdgeCutMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            request_id: try_from_required(&proto::ProofOfResponse {
                node: x.node1.clone(),  // Temporary mapping until we have proper RequestId serialization  
                timestamp: None,
                sequence: 0,
                response_type: None,
                ..Default::default()
            }).map_err(Self::Error::RequestId)?,
            node1: x.node1.clone(),
            node2: x.node2.clone(),
        })
    }
}

// SyncMessage conversion
impl From<&SyncMessage> for proto::SyncMessage {
    fn from(x: &SyncMessage) -> Self {
        Self {
            next_expected: MF::some(&to_proto_timestamp(&x.next_expected)),
            has_late_payment: x.has_late_payment,
            ..Default::default()
        }
    }
}

impl TryFrom<&proto::SyncMessage> for SyncMessage {
    type Error = ParseSyncMessageError;
    fn try_from(x: &proto::SyncMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            request_id: try_from_required(&proto::ProofOfResponse::default()) // Placeholder
                .map_err(Self::Error::RequestId)?,
            timestamp: Utc::now(),
            next_expected: from_proto_timestamp(
                x.next_expected.as_ref().ok_or_else(|| Self::Error::NextExpected(ParseRequiredError::new()))?
            ),
            has_late_payment: x.has_late_payment,
        })
    }
}

// PaymentMessage conversion
impl From<&PaymentMessage> for proto::PaymentMessage {
    fn from(x: &PaymentMessage) -> Self {
        Self {
            amount: x.amount,
            latency_so_far_ms: duration_to_millis(x.latency_so_far),
            ..Default::default()
        }
    }
}

impl TryFrom<&proto::PaymentMessage> for PaymentMessage {
    type Error = ParsePaymentMessageError;
    fn try_from(x: &proto::PaymentMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            request_id: try_from_required(&proto::ProofOfResponse::default()) // Placeholder
                .map_err(Self::Error::RequestId)?,
            amount: x.amount,
            latency_so_far: millis_to_duration(x.latency_so_far_ms),
        })
    }
}

// AckMessage conversion
impl From<&AckMessage> for proto::AckMessage {
    fn from(_x: &AckMessage) -> Self {
        Self::default()
    }
}

impl TryFrom<&proto::AckMessage> for AckMessage {
    type Error = ParseAckMessageError;
    fn try_from(_x: &proto::AckMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            request_id: try_from_required(&proto::ProofOfResponse::default()) // Placeholder
                .map_err(Self::Error::RequestId)?,
        })
    }
}

// NodeIdMessage conversion
impl From<&NodeIdMessage> for proto::NodeIdMessage {
    fn from(x: &NodeIdMessage) -> Self {
        Self {
            node_name: x.node_name.clone(),
            node_version: x.node_version,
            ..Default::default()
        }
    }
}

impl TryFrom<&proto::NodeIdMessage> for NodeIdMessage {
    type Error = ParseNodeIdMessageError;
    fn try_from(x: &proto::NodeIdMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            node_name: x.node_name.clone(),
            node_version: x.node_version,
        })
    }
}

// PorMessage conversion
impl From<&PorMessage> for proto::ProofOfResponse {
    fn from(x: &PorMessage) -> Self {
        let mut msg = Self::default();
        msg.response_type = Some(match x {
            PorMessage::Request(m) => ResponseType::Request(m.into()),
            PorMessage::Response(m) => ResponseType::Response(m.into()),
            PorMessage::EdgeCut(m) => ResponseType::EdgeCut(m.into()),
            PorMessage::Sync(m) => ResponseType::Sync(m.into()),
            PorMessage::Payment(m) => ResponseType::Payment(m.into()),
            PorMessage::Ack(m) => ResponseType::Ack(m.into()),
            PorMessage::NodeId(m) => ResponseType::NodeId(m.into()),
        });
        msg
    }
}

impl TryFrom<&proto::ProofOfResponse> for PorMessage {
    type Error = ParsePorMessageError;
    fn try_from(x: &proto::ProofOfResponse) -> Result<Self, Self::Error> {
        use proto::proof_of_response::ResponseType;
        Ok(match x.response_type.as_ref().ok_or(Self::Error::NoResponseType)? {
            ResponseType::Request(m) => Self::Request(m.try_into().map_err(Self::Error::Request)?),
            ResponseType::Response(m) => Self::Response(m.try_into().map_err(Self::Error::Response)?),
            ResponseType::EdgeCut(m) => Self::EdgeCut(m.try_into().map_err(Self::Error::EdgeCut)?),
            ResponseType::Sync(m) => Self::Sync(m.try_into().map_err(Self::Error::Sync)?),
            ResponseType::Payment(m) => Self::Payment(m.try_into().map_err(Self::Error::Payment)?),
            ResponseType::Ack(m) => Self::Ack(m.try_into().map_err(Self::Error::Ack)?),
            ResponseType::NodeId(m) => Self::NodeId(m.try_into().map_err(Self::Error::NodeId)?),
        })
    }
}

// Error types
#[derive(thiserror::Error, Debug)]
pub enum ParseProofOfResponseError {
    #[error("timestamp {0}")]
    Timestamp(ParseRequiredError<ParseTimestampError>),
}

#[derive(thiserror::Error, Debug)]
pub enum ParseRequestMessageError {
    #[error("request_id {0}")]
    RequestId(ParseRequiredError<ParseProofOfResponseError>),
}

#[derive(thiserror::Error, Debug)]
pub enum ParseResponseMessageError {
    #[error("request_id {0}")]
    RequestId(ParseRequiredError<ParseProofOfResponseError>),
}

#[derive(thiserror::Error, Debug)]
pub enum ParseEdgeCutMessageError {
    #[error("request_id {0}")]
    RequestId(ParseRequiredError<ParseProofOfResponseError>),
}

#[derive(thiserror::Error, Debug)]
pub enum ParseSyncMessageError {
    #[error("request_id {0}")]
    RequestId(ParseRequiredError<ParseProofOfResponseError>),
    #[error("next_expected {0}")]
    NextExpected(ParseRequiredError<ParseTimestampError>),
}

#[derive(thiserror::Error, Debug)]
pub enum ParsePaymentMessageError {
    #[error("request_id {0}")]
    RequestId(ParseRequiredError<ParseProofOfResponseError>),
}

#[derive(thiserror::Error, Debug)]
pub enum ParseAckMessageError {
    #[error("request_id {0}")]
    RequestId(ParseRequiredError<ParseProofOfResponseError>),
}

#[derive(thiserror::Error, Debug)]
pub enum ParseNodeIdMessageError {}

#[derive(thiserror::Error, Debug)]
pub enum ParsePorMessageError {
    #[error("no response_type field")]
    NoResponseType,
    #[error("request {0}")]
    Request(ParseRequestMessageError),
    #[error("response {0}")]
    Response(ParseResponseMessageError),
    #[error("edge_cut {0}")]
    EdgeCut(ParseEdgeCutMessageError),
    #[error("sync {0}")]
    Sync(ParseSyncMessageError),
    #[error("payment {0}")]
    Payment(ParsePaymentMessageError),
    #[error("ack {0}")]
    Ack(ParseAckMessageError),
    #[error("node_id {0}")]
    NodeId(ParseNodeIdMessageError),
}