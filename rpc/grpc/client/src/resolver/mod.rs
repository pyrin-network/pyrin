use super::error::Result;
use core::fmt::Debug;
use pyrin_grpc_core::{
    ops::PyrindPayloadOps,
    protowire::{PyrindRequest, PyrindResponse},
};
use std::{sync::Arc, time::Duration};
use tokio::sync::oneshot;

pub(crate) mod id;
pub(crate) mod matcher;
pub(crate) mod queue;

pub(crate) trait Resolver: Send + Sync + Debug {
    fn register_request(&self, op: PyrindPayloadOps, request: &PyrindRequest) -> PyrindResponseReceiver;
    fn handle_response(&self, response: PyrindResponse);
    fn remove_expired_requests(&self, timeout: Duration);
}

pub(crate) type DynResolver = Arc<dyn Resolver>;

pub(crate) type PyrindResponseSender = oneshot::Sender<Result<PyrindResponse>>;
pub(crate) type PyrindResponseReceiver = oneshot::Receiver<Result<PyrindResponse>>;
