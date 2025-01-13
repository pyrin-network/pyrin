use crate::Notification;

pub type ChannelConnection = pyrin_notify::connection::ChannelConnection<Notification>;
pub use pyrin_notify::connection::ChannelType;
