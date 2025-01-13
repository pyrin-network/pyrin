use pyrin_notify::{collector::CollectorFrom, converter::ConverterFrom};
use pyrin_rpc_core::Notification;

pub type WrpcServiceConverter = ConverterFrom<Notification, Notification>;
pub type WrpcServiceCollector = CollectorFrom<WrpcServiceConverter>;
