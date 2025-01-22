use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use axum::http::Request;
use tower_http::request_id::{MakeRequestId, RequestId};

// A `MakeRequestId` that increments an atomic counter
#[derive(Clone, Default)]
pub struct PiAppMakeRequestId {
    counter: Arc<AtomicU64>,
}

impl MakeRequestId for PiAppMakeRequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        self.counter
            .fetch_add(1, Ordering::SeqCst)
            .to_string()
            .parse()
            .ok()
            .map(|v| RequestId::new(v))
    }
}
