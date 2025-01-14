use crate::{
    error::{Error, Result},
    resolver::{PyrindResponseReceiver, PyrindResponseSender, Resolver},
};
use pyrin_core::trace;
use pyrin_grpc_core::{
    ops::PyrindPayloadOps,
    protowire::{PyrindRequest, PyrindResponse},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Instant,
};
use tokio::sync::oneshot;

#[derive(Debug)]
struct Pending {
    timestamp: Instant,
    sender: PyrindResponseSender,
}

impl Pending {
    fn new(sender: PyrindResponseSender) -> Self {
        Self { timestamp: Instant::now(), sender }
    }
}

#[derive(Debug)]
pub(crate) struct IdResolver {
    pending_calls: Arc<Mutex<HashMap<u64, Pending>>>,
}

impl IdResolver {
    pub(crate) fn new() -> Self {
        Self { pending_calls: Arc::new(Mutex::new(HashMap::new())) }
    }
}

impl Resolver for IdResolver {
    fn register_request(&self, _: PyrindPayloadOps, request: &PyrindRequest) -> PyrindResponseReceiver {
        let (sender, receiver) = oneshot::channel::<Result<PyrindResponse>>();
        {
            let mut pending_calls = self.pending_calls.lock().unwrap();
            pending_calls.insert(request.id, Pending::new(sender));
            drop(pending_calls);
        }
        receiver
    }

    fn handle_response(&self, response: PyrindResponse) {
        match self.pending_calls.lock().unwrap().remove(&response.id) {
            Some(pending) => {
                trace!("[Resolver] handle_response has matching request with id {}", response.id);
                match pending.sender.send(Ok(response)) {
                    Ok(_) => {}
                    Err(err) => {
                        trace!("[Resolver] handle_response failed to send the response of a pending request: {:?}", err);
                    }
                }
            }
            None => {
                trace!("[Resolver] handle_response: response id {} has no pending request", response.id);
            }
        }
    }

    fn remove_expired_requests(&self, timeout: std::time::Duration) {
        let mut pending_calls = self.pending_calls.lock().unwrap();
        let mut purge = Vec::<u64>::new();
        for (id, pending) in pending_calls.iter() {
            if pending.timestamp.elapsed() > timeout {
                purge.push(*id);
            }
        }
        for id in purge.iter() {
            let pending = pending_calls.remove(id).expect("the pending request to remove does exist in the map");
            match pending.sender.send(Err(Error::Timeout)) {
                Ok(_) => {}
                Err(err) => {
                    trace!("[Resolver] the timeout monitor failed to send a timeout error: {:?}", err);
                }
            }
        }
    }
}
