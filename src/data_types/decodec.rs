use std::io::{Error, Read, Write};

pub trait Decodable: Sized {
    fn decode<S: Read>(stream: &mut S) -> Result<Self, Error>;
}

pub trait Encodable: Sized {
    fn encode<S: Write>(self: &Self, stream: &mut S) -> Result<(), Error>;
}
