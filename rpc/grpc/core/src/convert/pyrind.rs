use crate::protowire::{pyrind_request, PyrindRequest, PyrindResponse};

impl From<pyrind_request::Payload> for PyrindRequest {
    fn from(item: pyrind_request::Payload) -> Self {
        PyrindRequest { id: 0, payload: Some(item) }
    }
}

impl AsRef<PyrindRequest> for PyrindRequest {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<PyrindResponse> for PyrindResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

pub mod pyrind_request_convert {
    use crate::protowire::*;
    use pyrin_rpc_core::{RpcError, RpcResult};

    impl_into_pyrind_request!(Shutdown);
    impl_into_pyrind_request!(SubmitBlock);
    impl_into_pyrind_request!(GetBlockTemplate);
    impl_into_pyrind_request!(GetBlock);
    impl_into_pyrind_request!(GetInfo);

    impl_into_pyrind_request!(GetCurrentNetwork);
    impl_into_pyrind_request!(GetPeerAddresses);
    impl_into_pyrind_request!(GetSink);
    impl_into_pyrind_request!(GetMempoolEntry);
    impl_into_pyrind_request!(GetMempoolEntries);
    impl_into_pyrind_request!(GetConnectedPeerInfo);
    impl_into_pyrind_request!(AddPeer);
    impl_into_pyrind_request!(SubmitTransaction);
    impl_into_pyrind_request!(GetSubnetwork);
    impl_into_pyrind_request!(GetVirtualChainFromBlock);
    impl_into_pyrind_request!(GetBlocks);
    impl_into_pyrind_request!(GetBlockCount);
    impl_into_pyrind_request!(GetBlockDagInfo);
    impl_into_pyrind_request!(ResolveFinalityConflict);
    impl_into_pyrind_request!(GetHeaders);
    impl_into_pyrind_request!(GetUtxosByAddresses);
    impl_into_pyrind_request!(GetBalanceByAddress);
    impl_into_pyrind_request!(GetBalancesByAddresses);
    impl_into_pyrind_request!(GetSinkBlueScore);
    impl_into_pyrind_request!(Ban);
    impl_into_pyrind_request!(Unban);
    impl_into_pyrind_request!(EstimateNetworkHashesPerSecond);
    impl_into_pyrind_request!(GetMempoolEntriesByAddresses);
    impl_into_pyrind_request!(GetCoinSupply);
    impl_into_pyrind_request!(Ping);
    impl_into_pyrind_request!(GetMetrics);
    impl_into_pyrind_request!(GetServerInfo);
    impl_into_pyrind_request!(GetSyncStatus);
    impl_into_pyrind_request!(GetDaaScoreTimestampEstimate);

    impl_into_pyrind_request!(NotifyBlockAdded);
    impl_into_pyrind_request!(NotifyNewBlockTemplate);
    impl_into_pyrind_request!(NotifyUtxosChanged);
    impl_into_pyrind_request!(NotifyPruningPointUtxoSetOverride);
    impl_into_pyrind_request!(NotifyFinalityConflict);
    impl_into_pyrind_request!(NotifyVirtualDaaScoreChanged);
    impl_into_pyrind_request!(NotifyVirtualChainChanged);
    impl_into_pyrind_request!(NotifySinkBlueScoreChanged);

    macro_rules! impl_into_pyrind_request {
        ($name:tt) => {
            paste::paste! {
                impl_into_pyrind_request_ex!(pyrin_rpc_core::[<$name Request>],[<$name RequestMessage>],[<$name Request>]);
            }
        };
    }

    use impl_into_pyrind_request;

    macro_rules! impl_into_pyrind_request_ex {
        // ($($core_struct:ident)::+, $($protowire_struct:ident)::+, $($variant:ident)::+) => {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<&$core_struct> for pyrind_request::Payload {
                fn from(item: &$core_struct) -> Self {
                    Self::$variant(item.into())
                }
            }

            impl From<&$core_struct> for PyrindRequest {
                fn from(item: &$core_struct) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<$core_struct> for pyrind_request::Payload {
                fn from(item: $core_struct) -> Self {
                    Self::$variant((&item).into())
                }
            }

            impl From<$core_struct> for PyrindRequest {
                fn from(item: $core_struct) -> Self {
                    Self { id: 0, payload: Some((&item).into()) }
                }
            }

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&pyrind_request::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &pyrind_request::Payload) -> RpcResult<Self> {
                    if let pyrind_request::Payload::$variant(request) = item {
                        request.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&PyrindRequest> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &PyrindRequest) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("PyrinRequest".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }

            impl From<$protowire_struct> for PyrindRequest {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(pyrind_request::Payload::$variant(item)) }
                }
            }

            impl From<$protowire_struct> for pyrind_request::Payload {
                fn from(item: $protowire_struct) -> Self {
                    pyrind_request::Payload::$variant(item)
                }
            }
        };
    }
    use impl_into_pyrind_request_ex;
}

pub mod pyrind_response_convert {
    use crate::protowire::*;
    use pyrin_rpc_core::{RpcError, RpcResult};

    impl_into_pyrind_response!(Shutdown);
    impl_into_pyrind_response!(SubmitBlock);
    impl_into_pyrind_response!(GetBlockTemplate);
    impl_into_pyrind_response!(GetBlock);
    impl_into_pyrind_response!(GetInfo);
    impl_into_pyrind_response!(GetCurrentNetwork);

    impl_into_pyrind_response!(GetPeerAddresses);
    impl_into_pyrind_response!(GetSink);
    impl_into_pyrind_response!(GetMempoolEntry);
    impl_into_pyrind_response!(GetMempoolEntries);
    impl_into_pyrind_response!(GetConnectedPeerInfo);
    impl_into_pyrind_response!(AddPeer);
    impl_into_pyrind_response!(SubmitTransaction);
    impl_into_pyrind_response!(GetSubnetwork);
    impl_into_pyrind_response!(GetVirtualChainFromBlock);
    impl_into_pyrind_response!(GetBlocks);
    impl_into_pyrind_response!(GetBlockCount);
    impl_into_pyrind_response!(GetBlockDagInfo);
    impl_into_pyrind_response!(ResolveFinalityConflict);
    impl_into_pyrind_response!(GetHeaders);
    impl_into_pyrind_response!(GetUtxosByAddresses);
    impl_into_pyrind_response!(GetBalanceByAddress);
    impl_into_pyrind_response!(GetBalancesByAddresses);
    impl_into_pyrind_response!(GetSinkBlueScore);
    impl_into_pyrind_response!(Ban);
    impl_into_pyrind_response!(Unban);
    impl_into_pyrind_response!(EstimateNetworkHashesPerSecond);
    impl_into_pyrind_response!(GetMempoolEntriesByAddresses);
    impl_into_pyrind_response!(GetCoinSupply);
    impl_into_pyrind_response!(Ping);
    impl_into_pyrind_response!(GetMetrics);
    impl_into_pyrind_response!(GetServerInfo);
    impl_into_pyrind_response!(GetSyncStatus);
    impl_into_pyrind_response!(GetDaaScoreTimestampEstimate);

    impl_into_pyrind_notify_response!(NotifyBlockAdded);
    impl_into_pyrind_notify_response!(NotifyNewBlockTemplate);
    impl_into_pyrind_notify_response!(NotifyUtxosChanged);
    impl_into_pyrind_notify_response!(NotifyPruningPointUtxoSetOverride);
    impl_into_pyrind_notify_response!(NotifyFinalityConflict);
    impl_into_pyrind_notify_response!(NotifyVirtualDaaScoreChanged);
    impl_into_pyrind_notify_response!(NotifyVirtualChainChanged);
    impl_into_pyrind_notify_response!(NotifySinkBlueScoreChanged);

    impl_into_pyrind_notify_response!(NotifyUtxosChanged, StopNotifyingUtxosChanged);
    impl_into_pyrind_notify_response!(NotifyPruningPointUtxoSetOverride, StopNotifyingPruningPointUtxoSetOverride);

    macro_rules! impl_into_pyrind_response {
        ($name:tt) => {
            paste::paste! {
                impl_into_pyrind_response_ex!(pyrin_rpc_core::[<$name Response>],[<$name ResponseMessage>],[<$name Response>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            paste::paste! {
                impl_into_pyrind_response_base!(pyrin_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>],[<$protowire_name Response>]);
            }
        };
    }
    use impl_into_pyrind_response;

    macro_rules! impl_into_pyrind_response_base {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<$core_struct>> for $protowire_struct {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    item.as_ref().map_err(|x| (*x).clone()).into()
                }
            }

            impl From<RpcError> for $protowire_struct {
                fn from(item: RpcError) -> Self {
                    let x: RpcResult<&$core_struct> = Err(item);
                    x.into()
                }
            }

            impl From<$protowire_struct> for pyrind_response::Payload {
                fn from(item: $protowire_struct) -> Self {
                    pyrind_response::Payload::$variant(item)
                }
            }

            impl From<$protowire_struct> for PyrindResponse {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(pyrind_response::Payload::$variant(item)) }
                }
            }
        };
    }
    use impl_into_pyrind_response_base;

    macro_rules! impl_into_pyrind_response_ex {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<&$core_struct>> for pyrind_response::Payload {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    pyrind_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<&$core_struct>> for PyrindResponse {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<RpcResult<$core_struct>> for pyrind_response::Payload {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    pyrind_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<$core_struct>> for PyrindResponse {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl_into_pyrind_response_base!($core_struct, $protowire_struct, $variant);

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&pyrind_response::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &pyrind_response::Payload) -> RpcResult<Self> {
                    if let pyrind_response::Payload::$variant(response) = item {
                        response.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&PyrindResponse> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &PyrindResponse) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("PyrinResponse".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }
        };
    }
    use impl_into_pyrind_response_ex;

    macro_rules! impl_into_pyrind_notify_response {
        ($name:tt) => {
            impl_into_pyrind_response!($name);

            paste::paste! {
                impl_into_pyrind_notify_response_ex!(pyrin_rpc_core::[<$name Response>],[<$name ResponseMessage>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            impl_into_pyrind_response!($core_name, $protowire_name);

            paste::paste! {
                impl_into_pyrind_notify_response_ex!(pyrin_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>]);
            }
        };
    }
    use impl_into_pyrind_notify_response;

    macro_rules! impl_into_pyrind_notify_response_ex {
        ($($core_struct:ident)::+, $protowire_struct:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl<T> From<Result<(), T>> for $protowire_struct
            where
                T: Into<RpcError>,
            {
                fn from(item: Result<(), T>) -> Self {
                    item
                        .map(|_| $($core_struct)::+{})
                        .map_err(|err| err.into()).into()
                }
            }

        };
    }
    use impl_into_pyrind_notify_response_ex;
}
