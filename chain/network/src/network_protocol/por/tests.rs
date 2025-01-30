use crate::network_protocol::testonly as data;
use crate::network_protocol::PeerMessage;
use crate::peer::testonly::{Event, PeerConfig, PeerHandle};
use crate::peer_manager::peer_manager_actor::Event as PME;
use crate::tcp;
use crate::testonly::make_rng;
use anyhow::Context as _;
use near_async::time;
use near_o11y::testonly::init_test_logger;
use std::sync::Arc;

use super::*;

/// Basic serialization tests
#[test]
fn test_por_request_serialization() {
    let request = PoRRequest {
        payload: b"test request".to_vec(),
    };
    
    // Test Borsh serialization/deserialization
    let bytes = request.try_to_vec().unwrap();
    let deserialized = PoRRequest::try_from_slice(&bytes).unwrap();
    assert_eq!(request, deserialized);
}

#[test]
fn test_por_response_serialization() {
    let response = PoRResponse {
        payload: b"test response".to_vec(),
    };
    
    // Test Borsh serialization/deserialization
    let bytes = response.try_to_vec().unwrap();
    let deserialized = PoRResponse::try_from_slice(&bytes).unwrap();
    assert_eq!(response, deserialized);
}

/// Tests two peers communicating using PoR messages
#[tokio::test]
async fn test_por_peer_communication() -> anyhow::Result<()> {
    init_test_logger();
    let mut rng = make_rng(89028037453);
    let mut clock = time::FakeClock::default();

    let chain = Arc::new(data::Chain::make(&mut clock, &mut rng, 12));
    let inbound_cfg = PeerConfig {
        chain: chain.clone(),
        network: chain.make_config(&mut rng),
        force_encoding: None,
    };
    let outbound_cfg = PeerConfig {
        chain: chain.clone(),
        network: chain.make_config(&mut rng),
        force_encoding: None,
    };

    // Create a loopback connection between the peers
    let (outbound_stream, inbound_stream) = tcp::Stream::loopback(inbound_cfg.id(), tcp::Tier::T2).await;
    let mut inbound = PeerHandle::start_endpoint(clock.clock(), inbound_cfg, inbound_stream).await;
    let mut outbound = PeerHandle::start_endpoint(clock.clock(), outbound_cfg, outbound_stream).await;

    // Complete handshake between peers
    outbound.complete_handshake().await;
    inbound.complete_handshake().await;

    let message_processed = |want| {
        move |ev| match ev {
            Event::Network(PME::MessageProcessed(_, got)) if got == want => Some(()),
            _ => None,
        }
    };

    // Test PoRRequest
    let mut events = inbound.events.from_now();
    let want = PeerMessage::PoRRequest(PoRRequest {
        payload: b"test request".to_vec(),
    });
    outbound.send(want.clone()).await;
    events.recv_until(message_processed(want)).await;

    // Test PoRResponse 
    let mut events = inbound.events.from_now();
    let want = PeerMessage::PoRResponse(PoRResponse {
        payload: b"test response".to_vec(),
    });
    outbound.send(want.clone()).await;
    events.recv_until(message_processed(want)).await;

    Ok(())
}