use std::io::ErrorKind;

use tokio::io::{AsyncReadExt, AsyncWriteExt, Error};

use super::decodec::{Decodable, Encodable, FixedSizeDecodable, FixedSizeEncodable};

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

impl Decodable for i32 {
    async fn decode<S: AsyncReadExt + Unpin>(stream: &mut S) -> Result<Self, Error> {
        let mut value = 0;
        let mut position = 0;
        loop {
            let current_byte = u8::fixed_decode(stream).await?;
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
    async fn decode<S: AsyncReadExt + Unpin>(stream: &mut S) -> Result<Self, Error> {
        let mut value = 0;
        let mut position = 0;
        loop {
            let current_byte = u8::fixed_decode(stream).await?;
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
    async fn encode<S: AsyncWriteExt + Unpin>(self: &Self, stream: &mut S) -> Result<usize, Error> {
        let mut value = *self;
        let mut byte_size = 0;
        loop {
            byte_size += 1;
            if (value & !i32::from(SEGMENT_BITS)) == 0 {
                return (value as u8).fixed_encode(stream).await.map(|_| byte_size);
            }
            (((value & i32::from(SEGMENT_BITS)) as u8) | CONTINUE_BIT)
                .fixed_encode(stream)
                .await?;
            value = value >> 7;
        }
    }
}

impl Encodable for i64 {
    async fn encode<S: AsyncWriteExt + Unpin>(self: &Self, stream: &mut S) -> Result<usize, Error> {
        let mut value = *self;
        let mut byte_size = 0;
        loop {
            byte_size += 1;
            let temp = !i64::from(SEGMENT_BITS);
            if (value & temp) == 0 {
                return (value as u8).fixed_encode(stream).await.map(|_| byte_size);
            }
            (((value & i64::from(SEGMENT_BITS)) as u8) | CONTINUE_BIT)
                .fixed_encode(stream)
                .await?;
            value = value >> 7;
        }
    }
}

impl FixedSizeDecodable<1> for u8 {
    async fn fixed_decode<S: AsyncReadExt + Unpin>(stream: &mut S) -> Result<Self, Error> {
        let mut buffer = [0];
        let amount_read = stream.read(&mut buffer).await?;
        if amount_read != 1 {
            return Err(Error::new(ErrorKind::InvalidData, "empty data for u8"));
        }
        return Ok(buffer[0]);
    }
}

impl FixedSizeEncodable<1> for u8 {
    async fn fixed_encode<S: AsyncWriteExt + Unpin>(
        self: &Self,
        stream: &mut S,
    ) -> Result<(), Error> {
        return stream.write(&self.to_be_bytes()).await.map(|_| ());
    }
}

impl FixedSizeDecodable<2> for u16 {
    async fn fixed_decode<S: AsyncReadExt + Unpin>(stream: &mut S) -> Result<Self, Error> {
        let mut buffer = [0; 2];
        let amount_read = stream.read(&mut buffer).await?;
        if amount_read != 2 {
            return Err(Error::new(ErrorKind::InvalidData, "empty data for u16"));
        }
        return Ok(u16::from_be_bytes(buffer));
    }
}

impl FixedSizeEncodable<16> for u128 {
    async fn fixed_encode<S: AsyncWriteExt + Unpin>(
        self: &Self,
        stream: &mut S,
    ) -> Result<(), Error> {
        return stream.write(&self.to_be_bytes()).await.map(|_| ());
    }
}

impl FixedSizeDecodable<16> for u128 {
    async fn fixed_decode<S: AsyncReadExt + Unpin>(stream: &mut S) -> Result<Self, Error> {
        let mut buffer = [0; 16];
        let amount_read = stream.read(&mut buffer).await?;
        if amount_read != 16 {
            return Err(Error::new(ErrorKind::InvalidData, "empty data for u128"));
        }
        return Ok(u128::from_be_bytes(buffer));
    }
}

impl FixedSizeDecodable<8> for i64 {
    async fn fixed_decode<S: AsyncReadExt + Unpin>(stream: &mut S) -> Result<Self, Error> {
        let mut buffer = [0; 8];
        let amount_read = stream.read(&mut buffer).await?;
        if amount_read != 8 {
            return Err(Error::new(ErrorKind::InvalidData, "empty data for i64"));
        }
        return Ok(i64::from_be_bytes(buffer));
    }
}

impl FixedSizeEncodable<8> for i64 {
    async fn fixed_encode<S: AsyncWriteExt + Unpin>(
        self: &Self,
        stream: &mut S,
    ) -> Result<(), Error> {
        return stream.write(&self.to_be_bytes()).await.map(|_| ());
    }
}

#[tokio::test]
async fn read_write_var_int() {
    let expected = 183944198i32;
    let (mut client, mut server) = tokio::io::duplex(100000);
    expected.encode(&mut client).await.unwrap();
    assert_eq!(expected, i32::decode(&mut server).await.unwrap());
}

#[tokio::test]
async fn read_write_var_long() {
    let expected = 183944198i64;
    let (mut client, mut server) = tokio::io::duplex(100000);
    expected.encode(&mut client).await.unwrap();
    assert_eq!(expected, i64::decode(&mut server).await.unwrap());
}
