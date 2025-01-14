use crate::from;
use crate::protowire;

// ----------------------------------------------------------------------------
// rpc_core to protowire
// ----------------------------------------------------------------------------

from!(item: pyrin_rpc_core::RpcError, protowire::RpcError, { Self { message: item.to_string() } });
from!(item: &pyrin_rpc_core::RpcError, protowire::RpcError, { Self { message: item.to_string() } });

// ----------------------------------------------------------------------------
// protowire to rpc_core
// ----------------------------------------------------------------------------

from!(item: &protowire::RpcError, pyrin_rpc_core::RpcError, { pyrin_rpc_core::RpcError::from(item.message.to_string()) });
