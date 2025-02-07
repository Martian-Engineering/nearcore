use crate::types::PeerManagerSenderForNetwork;
use actix::{Actor, AsyncContext, Context, Handler};
use near_async::time;
use std::time::Duration;

// GOAL: On startup:
// 1. Get the block height from the chain
// 2. Send the block height to another peer over the network

pub struct PorActor {
    // TODO:
}

impl Actor for PorActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        tracing::info!(target: "por", "PorActor started");
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        tracing::info!(target: "por", "PorActor stopped");
        actix::Running::Stop
    }
}
