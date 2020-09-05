#![feature(untagged_unions)]
#![feature(min_const_generics)]
#![feature(const_fn)]

pub mod driver_messages;
pub mod turret_messages;

mod udp_message;
pub use udp_message::{
    UDPMessageBuffer, UDPMessageCode, UDPMessageTuple, SIZE_MAX_UDP_MESSAGE_BUFFER, SIZE_MAX_UDP_MESSAGE_BUFFER_DATA,
    SIZE_UDP_MESSAGE_BUFFER_HEADER,
};

use std::io::{Error as IOError, ErrorKind as IOErrorKind, Result as IOResult};
use std::net::Ipv4Addr;

pub trait ITransportableMessage<T: Clone, const TSIZE: usize> {
    fn get_raw_buffer(&self) -> &[u8];
    fn get_raw_buffer_mut(&mut self) -> &mut [u8];
    fn get_cloned_structure(&self) -> T;
    fn get_structure_mut(&mut self) -> &mut T;
    fn get_message_code(&self) -> UDPMessageCode;

    fn copy_to_message_buffer(&self, origin_address: Ipv4Addr, message_buffer: &mut UDPMessageBuffer) {
        message_buffer.set_origin(origin_address);
        message_buffer.set_message_code(self.get_message_code());
        message_buffer.set_data(Some(self.get_raw_buffer()));
    }

    fn copy_from_message_buffer(&mut self, message_buffer: &UDPMessageBuffer) -> IOResult<Ipv4Addr> {
        if message_buffer.get_message_code() != self.get_message_code() {
            return Err(IOError::new(IOErrorKind::InvalidData, "Invalid Message Code!"));
        }

        if message_buffer.get_data_length() != TSIZE {
            return Err(IOError::new(IOErrorKind::InvalidData, "Invalid Data Length!"));
        }

        self.get_raw_buffer_mut().copy_from_slice(message_buffer.get_data_slice().unwrap());

        Ok(message_buffer.get_origin())
    }

    fn get_updated_structure(&mut self, message_buffer: &UDPMessageBuffer) -> IOResult<(Ipv4Addr, T)> {
        let origin_address = self.copy_from_message_buffer(message_buffer)?;

        Ok((origin_address, self.get_cloned_structure()))
    }

    fn updated_buffer_from_structure(&mut self, origin_address: Ipv4Addr, message_buffer: &mut UDPMessageBuffer, source: T) {
        *self.get_structure_mut() = source;
        self.copy_to_message_buffer(origin_address, message_buffer);
    }
}
