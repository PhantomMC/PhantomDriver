use std::{
    collections::VecDeque,
    io::{Error, Read, Write},
};

use super::decodec::{Decodable, Encodable};

impl Decodable for String {
    fn decode<S: Read>(stream: &mut S) -> Result<Self, Error> {
        return i32::decode(stream).and_then(|length| read_string_with_length(stream, length));
    }
}

fn read_string_with_length<S: Read>(stream: &mut S, length: i32) -> Result<String, Error> {
    let mut take = stream.take(length as u64);
    let mut output = String::new();
    return take.read_to_string(&mut output).map(|_| output);
}

impl Encodable for String {
    fn encode<S: Write>(self: &Self, stream: &mut S) -> Result<(), Error> {
        let bytes = self.as_bytes();
        return (bytes.len() as i32)
            .encode(stream)
            .and_then(|_| stream.write_all(bytes));
    }
}

#[test]
fn encode_decode() {
    let expected = String::from("Hello world!");
    let mut stream: VecDeque<u8> = VecDeque::new();
    expected.encode(&mut stream).unwrap();
    assert_eq!(expected, String::decode(&mut stream).unwrap())
}
