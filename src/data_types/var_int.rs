use std::{
    collections::VecDeque,
    io::{Error, ErrorKind, Read, Write},
};

use super::decodec::{Decodable, Encodable};

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

impl Decodable for i32 {
    fn decode<S: Read>(stream: &mut S) -> Result<Self, Error> {
        let mut value = 0;
        let mut position = 0;
        loop {
            let mut current_byte = [0; 1];
            let read_amount = stream.read(&mut current_byte)?;
            if read_amount == 0 {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Unexpected interupt for reading varint",
                ));
            }
            value |= i32::from(current_byte[0] & SEGMENT_BITS) << position;
            if (current_byte[0] & CONTINUE_BIT) == 0 {
                break;
            };
            position += 7;
            if position >= 32 {
                return Err(Error::new(ErrorKind::InvalidData, "Var int is too big"));
            }
        }
        return Ok(value);
    }
}

impl Decodable for i64 {
    fn decode<S: Read>(stream: &mut S) -> Result<Self, Error> {
        let mut value = 0;
        let mut position = 0;
        loop {
            let mut current_byte = [0; 1];
            let read_amount = stream.read(&mut current_byte)?;
            if read_amount == 0 {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Unexpected interupt for reading varint",
                ));
            }
            value |= i64::from(current_byte[0] & SEGMENT_BITS) << position;
            if (current_byte[0] & CONTINUE_BIT) == 0 {
                break;
            };
            position += 7;
            if position >= 64 {
                return Err(Error::new(ErrorKind::InvalidData, "Var int is too big"));
            }
        }
        return Ok(value);
    }
}

impl Encodable for i32 {
    fn encode<S: Write>(self: &Self, stream: &mut S) -> Result<(), Error> {
        let mut value = *self;
        loop {
            if (value & !i32::from(SEGMENT_BITS)) == 0 {
                return stream.write(&[value as u8; 1]).map(|_| ());
            }
            let err = stream
                .write(&[((value & i32::from(SEGMENT_BITS)) as u8) | CONTINUE_BIT; 1])
                .err();
            if err.is_some() {
                return Err(err.unwrap());
            }
            value = value >> 7;
        }
    }
}

impl Encodable for i64 {
    fn encode<S: Write>(self: &Self, stream: &mut S) -> Result<(), Error> {
        let mut value = *self;
        loop {
            let temp = !i64::from(SEGMENT_BITS);
            if (value & temp) == 0 {
                return stream.write(&[value as u8; 1]).map(|_| ());
            }
            let err = stream
                .write(&[((value & i64::from(SEGMENT_BITS)) as u8) | CONTINUE_BIT; 1])
                .err();
            if err.is_some() {
                return Err(err.unwrap());
            }
            value = value >> 7;
        }
    }
}

#[test]
fn read_write_var_int() {
    let expected = 183944198i32;
    let mut stream: VecDeque<u8> = VecDeque::new();
    expected.encode(&mut stream).unwrap();
    assert_eq!(expected, i32::decode(&mut stream).unwrap());
}

#[test]
fn read_write_var_long() {
    let expected = 183944198i64;
    let mut stream: VecDeque<u8> = VecDeque::new();
    expected.encode(&mut stream).unwrap();
    let stream_size = stream.len();
    println!("stream size: {stream_size}");
    assert_eq!(expected, i64::decode(&mut stream).unwrap());
}
