use std::borrow::Borrow;
use std::cmp::PartialEq;
use std::iter::IntoIterator;
use std::net::Ipv4Addr;
use std::ops::{Index, Range, RangeFull, RangeTo};
use std::vec::IntoIter;

pub const SIZE_MAX_UDP_MESSAGE_BUFFER: usize = 1460;
pub const SIZE_UDP_MESSAGE_BUFFER_HEADER: usize = 8;
pub const SIZE_MAX_UDP_MESSAGE_BUFFER_DATA: usize = SIZE_MAX_UDP_MESSAGE_BUFFER - SIZE_UDP_MESSAGE_BUFFER_HEADER;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum UDPMessageCode {
    Input(u8),
    Output(u8),
}

impl UDPMessageCode {
    pub fn as_u16(self) -> u16 {
        match self {
            UDPMessageCode::Input(val) => 0x0000 | val as u16,
            UDPMessageCode::Output(val) => 0xFF00 | val as u16,
        }
    }

    pub fn from_u16(source: u16) -> Option<Self> {
        match source & 0xFF00 {
            0x0000 => Some(UDPMessageCode::Input((source & 0x00FF) as u8)),
            0xFF00 => Some(UDPMessageCode::Output((source & 0x00FF) as u8)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UDPMessageBuffer {
    raw: [u8; SIZE_MAX_UDP_MESSAGE_BUFFER],
}

impl Default for UDPMessageBuffer {
    fn default() -> Self {
        Self {
            raw: [0; SIZE_MAX_UDP_MESSAGE_BUFFER],
        }
    }
}

impl From<(Ipv4Addr, UDPMessageCode, Option<Vec<u8>>)> for UDPMessageBuffer {
    fn from(source_tuple: (Ipv4Addr, UDPMessageCode, Option<Vec<u8>>)) -> Self {
        let mut instance = UDPMessageBuffer::default();

        match source_tuple.2 {
            Some(source_data) => {
                let source_data = &source_data[..];

                if !instance.update_message(source_tuple.0, source_tuple.1, Some(source_data)) {
                    panic!("Invalid size of Data!");
                }
            }
            None => {
                instance.update_message(source_tuple.0, source_tuple.1, None);
            }
        }

        instance
    }
}

impl Into<(Ipv4Addr, UDPMessageCode, Option<Vec<u8>>)> for UDPMessageBuffer {
    fn into(self) -> (Ipv4Addr, UDPMessageCode, Option<Vec<u8>>) {
        let data_vec = match self.get_data_slice() {
            None => None,
            Some(data_slice) => Some(data_slice.to_vec()),
        };

        (self.get_origin(), self.get_message_code(), data_vec)
    }
}

impl Borrow<[u8; SIZE_MAX_UDP_MESSAGE_BUFFER]> for UDPMessageBuffer {
    fn borrow(&self) -> &[u8; SIZE_MAX_UDP_MESSAGE_BUFFER] {
        &self.raw
    }
}

impl Index<usize> for UDPMessageBuffer {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.raw[index]
    }
}

impl Index<RangeTo<usize>> for UDPMessageBuffer {
    type Output = [u8];

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.raw[index]
    }
}

impl Index<RangeFull> for UDPMessageBuffer {
    type Output = [u8];

    fn index(&self, _: RangeFull) -> &Self::Output {
        &self.raw[..(self.len())]
    }
}

impl Index<Range<usize>> for UDPMessageBuffer {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.raw[index]
    }
}

impl IntoIterator for UDPMessageBuffer {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self[..].to_vec().into_iter()
    }
}

impl UDPMessageBuffer {
    fn slice_has_header(slice: &[u8]) -> bool {
        slice.len() >= SIZE_UDP_MESSAGE_BUFFER_HEADER
    }

    fn get_slice_data_length(slice: &[u8]) -> Option<usize> {
        if !UDPMessageBuffer::slice_has_header(slice) {
            return None;
        }

        Some((slice[6] as usize) | ((slice[7] as usize) << 8))
    }

    fn get_slice_message_code(slice: &[u8]) -> Option<UDPMessageCode> {
        if !UDPMessageBuffer::slice_has_header(slice) {
            return None;
        }

        UDPMessageCode::from_u16((slice[4] as u16) | ((slice[5] as u16) << 8))
    }

    pub fn new(origin: Ipv4Addr, message_code: UDPMessageCode, data: Option<&[u8]>) -> Self {
        let mut instance = UDPMessageBuffer::default();

        match data {
            Some(source_data) => {
                let source_data = &source_data[..];

                if !instance.update_message(origin, message_code, Some(source_data)) {
                    panic!("Invalid size of Data!");
                }
            }
            None => {
                instance.update_message(origin, message_code, None);
            }
        }

        instance
    }

    pub fn into_inner(self) -> [u8; SIZE_MAX_UDP_MESSAGE_BUFFER] {
        self.raw
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.raw[..self.len() as usize].to_vec()
    }

    pub fn update_from_slice(&mut self, slice: &[u8]) -> bool {
        let slice_length = slice.len();

        if slice_length > SIZE_MAX_UDP_MESSAGE_BUFFER {
            return false;
        }

        match UDPMessageBuffer::get_slice_data_length(slice) {
            None => false,
            Some(length) => {
                if length != (slice_length - SIZE_UDP_MESSAGE_BUFFER_HEADER) {
                    return false;
                }

                self.raw[..slice_length].copy_from_slice(slice);

                true
            }
        }
    }

    pub fn set_origin(&mut self, origin: Ipv4Addr) {
        let origin_u32: u32 = origin.into();
        self.raw[0..4].copy_from_slice(&origin_u32.to_le_bytes()[..]);
    }

    pub fn set_message_code(&mut self, message_code: UDPMessageCode) {
        self.raw[4..6].copy_from_slice(&message_code.as_u16().to_le_bytes()[..]);
    }

    pub fn set_data(&mut self, data: Option<&[u8]>) -> bool {
        for i in SIZE_UDP_MESSAGE_BUFFER_HEADER..SIZE_MAX_UDP_MESSAGE_BUFFER_DATA {
            self.raw[i] = 0;
        }

        match data {
            None => {
                self.raw[6..8].copy_from_slice(&0u16.to_le_bytes()[..]);

                true
            }
            Some(data_bytes) => {
                if data_bytes.len() > SIZE_MAX_UDP_MESSAGE_BUFFER_DATA {
                    return false;
                }

                let data_length = data_bytes.len() as u16;
                self.raw[6..8].copy_from_slice(&data_length.to_le_bytes()[..]);
                self.raw[8..(8 + data_bytes.len())].copy_from_slice(data_bytes);

                true
            }
        }
    }

    pub fn update_message(&mut self, origin: Ipv4Addr, message_code: UDPMessageCode, data: Option<&[u8]>) -> bool {
        self.set_origin(origin);
        self.set_message_code(message_code);
        self.set_data(data)
    }

    pub fn len(&self) -> usize {
        self.get_data_length() + SIZE_UDP_MESSAGE_BUFFER_HEADER
    }

    pub fn get_data_length(&self) -> usize {
        UDPMessageBuffer::get_slice_data_length(&self.raw).unwrap()
    }

    pub fn get_message_code(&self) -> UDPMessageCode {
        UDPMessageBuffer::get_slice_message_code(&self.raw).unwrap()
    }

    pub fn get_origin(&self) -> Ipv4Addr {
        let origin_u32 = (self.raw[0] as u32)
            | ((self.raw[1] as u32) << 8)
            | ((self.raw[2] as u32) << 16)
            | ((self.raw[3] as u32) << 24);
        Ipv4Addr::from(origin_u32)
    }

    pub fn get_data_slice(&self) -> Option<&[u8]> {
        let data_length = self.get_data_length();

        if data_length == 0 {
            return None;
        }

        Some(&self.raw[8..(8 + data_length as usize)])
    }

    pub fn into_tuple(self) -> (Ipv4Addr, UDPMessageCode, Option<Vec<u8>>) {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_from_invalid_slice() {
        let bad_buffer = vec![0, 0, 0, 0, 0, 0, 0, 0, 1];
        let mut message_buffer = UDPMessageBuffer::default();
        assert!(!message_buffer.update_from_slice(&bad_buffer[..]));
    }

    #[test]
    fn test_update_from_valid_slice() {
        let good_buffer = vec![0, 0, 0, 0, 0, 0, 1, 0, 1];
        let mut message_buffer = UDPMessageBuffer::default();
        assert!(message_buffer.update_from_slice(&good_buffer[..]));
    }

    #[test]
    fn test_set_length() {
        let data = vec![1, 2, 3];
        let mut message_buffer = UDPMessageBuffer::default();
        assert!(message_buffer.set_data(Some(&data)));
        assert_eq!(message_buffer.get_data_length(), 3);
    }

    #[test]
    fn test_set_message_code() {
        let good_buffer = vec![0, 0, 0, 0, 0, 0, 1, 0, 1];
        let mut message_buffer = UDPMessageBuffer::default();
        assert!(message_buffer.update_from_slice(&good_buffer[..]));
        assert_eq!(message_buffer.get_message_code(), UDPMessageCode::Input(0));
        message_buffer.set_message_code(UDPMessageCode::Output(11));
        assert_eq!(message_buffer.get_message_code(), UDPMessageCode::Output(11));
        assert_eq!(&message_buffer[..], (vec![0, 0, 0, 0, 11, 0xFF, 1, 0, 1]).as_slice());
    }

    #[test]
    fn test_zero_length_data() {
        let expected_ip: Ipv4Addr = "127.0.0.1".parse().unwrap();
        let good_buffer = vec![1, 0, 0, 127, 1, 0, 0, 0];
        let mut message_buffer = UDPMessageBuffer::default();
        assert!(message_buffer.update_from_slice(&good_buffer[..]));
        assert_eq!(message_buffer.get_data_length(), 0);
        assert_eq!(message_buffer.get_origin(), expected_ip);
        assert_eq!(message_buffer.get_message_code(), UDPMessageCode::Input(1));
    }

    #[test]
    fn test_from_tuple() {
        let origin_address = Ipv4Addr::LOCALHOST;
        let origin_message_code = UDPMessageCode::Output(0xC3);
        let origin_data_length = 2;
        let origin_data = vec![1, 2];
        let tuple = (origin_address, origin_message_code, Some(origin_data.clone()));
        let message_buffer = UDPMessageBuffer::from(tuple);
        assert_eq!(message_buffer.get_origin(), origin_address);
        assert_eq!(message_buffer.get_message_code(), origin_message_code);
        assert_eq!(message_buffer.get_data_length(), origin_data_length);
        assert_eq!(message_buffer.get_data_slice(), Some(&origin_data.to_vec()[..]));
    }

    #[test]
    fn test_into_tuple() {
        let origin_address = Ipv4Addr::LOCALHOST;
        let origin_message_code = UDPMessageCode::Output(0xC3);
        let origin_data_length = 2;
        let origin_data = vec![1, 2];
        let message_buffer = UDPMessageBuffer::new(origin_address, origin_message_code, Some(&origin_data[..]));
        let tuple = message_buffer.into_tuple();
        assert!(tuple.2.is_some());
        let extracted_data = tuple.2.unwrap();
        assert_eq!(tuple.0, origin_address);
        assert_eq!(tuple.1, origin_message_code);
        assert_eq!(extracted_data.len(), origin_data_length);
        assert_eq!(extracted_data[0], origin_data[0]);
        assert_eq!(extracted_data[1], origin_data[1]);
    }

    #[test]
    fn test_full_comparison() {
        let mut origin_buffer = UDPMessageBuffer::default();
        let origin_address = "127.0.0.1".parse().unwrap();
        let origin_message_code = UDPMessageCode::Output(0xAC);
        let origin_data = vec![1, 2, 3, 4, 5];
        origin_buffer.update_message(origin_address, origin_message_code, Some(&origin_data));
        let origin_raw_buffer = origin_buffer.into_vec();
        let mut received_buffer = UDPMessageBuffer::default();
        assert!(received_buffer.update_from_slice(&origin_raw_buffer));
        assert_eq!(received_buffer.get_origin(), origin_address);
        assert_eq!(received_buffer.get_message_code(), origin_message_code);
        assert_eq!(received_buffer.get_data_length(), origin_data.len());
        assert_eq!(received_buffer.get_data_slice().unwrap(), origin_data);
        assert_eq!(received_buffer[..], origin_raw_buffer[..]);
    }
}
