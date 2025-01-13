use pyrin_notify::{collector::CollectorFrom, converter::ConverterFrom};
use pyrin_rpc_core::Notification;

pub type GrpcServiceConverter = ConverterFrom<Notification, Notification>;
pub type GrpcServiceCollector = CollectorFrom<GrpcServiceConverter>;
