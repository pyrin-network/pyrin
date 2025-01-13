use super::method::{DropFn, Method, MethodTrait, RoutingPolicy};
use crate::{
    connection::Connection,
    connection_handler::ServerContext,
    error::{GrpcServerError, GrpcServerResult},
};
use pyrin_grpc_core::{
    ops::PyrindPayloadOps,
    protowire::{PyrindRequest, PyrindResponse},
};
use std::fmt::Debug;
use std::{collections::HashMap, sync::Arc};

pub type PyrindMethod = Method<ServerContext, Connection, PyrindRequest, PyrindResponse>;
pub type DynPyrindMethod = Arc<dyn MethodTrait<ServerContext, Connection, PyrindRequest, PyrindResponse>>;
pub type PyrindDropFn = DropFn<PyrindRequest, PyrindResponse>;
pub type PyrindRoutingPolicy = RoutingPolicy<PyrindRequest, PyrindResponse>;

/// An interface providing methods implementations and a fallback "not implemented" method
/// actually returning a message with a "not implemented" error.
///
/// The interface can provide a method clone for every [`PyrindPayloadOps`] variant for later
/// processing of related requests.
///
/// It is also possible to directly let the interface itself process a request by invoking
/// the `call()` method.
pub struct Interface {
    server_ctx: ServerContext,
    methods: HashMap<PyrindPayloadOps, DynPyrindMethod>,
    method_not_implemented: DynPyrindMethod,
}

impl Interface {
    pub fn new(server_ctx: ServerContext) -> Self {
        let method_not_implemented = Arc::new(Method::new(|_, _, pyrind_request: PyrindRequest| {
            Box::pin(async move {
                match pyrind_request.payload {
                    Some(ref request) => Ok(PyrindResponse {
                        id: pyrind_request.id,
                        payload: Some(PyrindPayloadOps::from(request).to_error_response(GrpcServerError::MethodNotImplemented.into())),
                    }),
                    None => Err(GrpcServerError::InvalidRequestPayload),
                }
            })
        }));
        Self { server_ctx, methods: Default::default(), method_not_implemented }
    }

    pub fn method(&mut self, op: PyrindPayloadOps, method: PyrindMethod) {
        let method: DynPyrindMethod = Arc::new(method);
        if self.methods.insert(op, method).is_some() {
            panic!("RPC method {op:?} is declared multiple times")
        }
    }

    pub fn replace_method(&mut self, op: PyrindPayloadOps, method: PyrindMethod) {
        let method: DynPyrindMethod = Arc::new(method);
        let _ = self.methods.insert(op, method);
    }

    pub fn set_method_properties(
        &mut self,
        op: PyrindPayloadOps,
        tasks: usize,
        queue_size: usize,
        routing_policy: PyrindRoutingPolicy,
    ) {
        self.methods.entry(op).and_modify(|x| {
            let method: Method<ServerContext, Connection, PyrindRequest, PyrindResponse> =
                Method::with_properties(x.method_fn(), tasks, queue_size, routing_policy);
            let method: Arc<dyn MethodTrait<ServerContext, Connection, PyrindRequest, PyrindResponse>> = Arc::new(method);
            *x = method;
        });
    }

    pub async fn call(
        &self,
        op: &PyrindPayloadOps,
        connection: Connection,
        request: PyrindRequest,
    ) -> GrpcServerResult<PyrindResponse> {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).call(self.server_ctx.clone(), connection, request).await
    }

    pub fn get_method(&self, op: &PyrindPayloadOps) -> DynPyrindMethod {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).clone()
    }
}

impl Debug for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interface").finish()
    }
}
