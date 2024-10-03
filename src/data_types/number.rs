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
            let current_byte = u8::decode(stream)?;
            value |= i32::from(current_byte & SEGMENT_BITS) << position;
            if (current_byte & CONTINUE_BIT) == 0 {
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
            let current_byte = u8::decode(stream)?;
            value |= i64::from(current_byte & SEGMENT_BITS) << position;
            if (current_byte & CONTINUE_BIT) == 0 {
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

impl Decodable for u8 {
    fn decode<S: Read>(stream: &mut S) -> Result<Self, Error> {
        let mut buffer = [0];
        let amount_read = stream.read(&mut buffer)?;
        if amount_read != 1 {
            return Err(Error::new(ErrorKind::InvalidData, "empty data for u8"));
        }
        return Ok(buffer[0]);
    }
}

impl Decodable for u16 {
    fn decode<S: Read>(stream: &mut S) -> Result<Self, Error> {
        let mut buffer = [0; 2];
        let amount_read = stream.read(&mut buffer)?;
        if amount_read != 2 {
            return Err(Error::new(ErrorKind::InvalidData, "empty data for u16"));
        }
        return Ok(u16::from_be_bytes(buffer));
    }
}

impl Encodable for u128 {
    fn encode<S: Write>(self: &Self, stream: &mut S) -> Result<(), Error> {
        return stream.write_all(&self.to_be_bytes());
    }
}

impl Decodable for u128 {
    fn decode<S: Read>(stream: &mut S) -> Result<Self, Error> {
        let mut buffer = [0; 16];
        let amount_read = stream.read(&mut buffer)?;
        if amount_read != 16 {
            return Err(Error::new(ErrorKind::InvalidData, "empty data for u128"));
        }
        return Ok(u128::from_be_bytes(buffer));
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
    assert_eq!(expected, i64::decode(&mut stream).unwrap());
}
