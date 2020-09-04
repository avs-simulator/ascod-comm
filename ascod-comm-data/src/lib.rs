#![feature(untagged_unions)]

pub mod driver_messages;
pub mod turret_messages;

mod udp_message;
pub use udp_message::{
    UDPMessageBuffer, UDPMessageCode, UDPMessageTuple, SIZE_MAX_UDP_MESSAGE_BUFFER, SIZE_MAX_UDP_MESSAGE_BUFFER_DATA,
    SIZE_UDP_MESSAGE_BUFFER_HEADER,
};
