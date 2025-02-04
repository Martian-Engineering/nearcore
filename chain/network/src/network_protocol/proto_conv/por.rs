use crate::network_protocol::proto::ProofOfResponse;
use crate::por::PorMessage;

impl From<&PorMessage> for ProofOfResponse {
    fn from(x: &PorMessage) -> Self {
        Self {
            content: x.content.clone(),
            ..Default::default()
        }
    }
}

impl From<&ProofOfResponse> for PorMessage {
    fn from(x: &ProofOfResponse) -> Self {
        Self {
            content: x.content.clone(),
        }
    }
}
