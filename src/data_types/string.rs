use tokio::io::{AsyncReadExt, AsyncWriteExt, Error};

use super::decodec::{Decodable, Encodable};

impl Decodable for String {
    async fn decode<S: AsyncReadExt + Unpin>(stream: &mut S) -> Result<Self, Error> {
        let length = i32::decode(stream).await?;
        return read_string_with_length(stream, length).await;
    }
}

async fn read_string_with_length<S: AsyncReadExt + Unpin>(
    stream: &mut S,
    length: i32,
) -> Result<String, Error> {
    let mut take = stream.take(length as u64);
    let mut output = String::new();
    return take.read_to_string(&mut output).await.map(|_| output);
}

impl Encodable for String {
    async fn encode<S: AsyncWriteExt + Unpin>(self: &Self, stream: &mut S) -> Result<usize, Error> {
        let bytes = self.as_bytes();
        let bytes_len = bytes.len();
        let var_int_size = (bytes_len as i32).encode(stream).await?;
        stream.write_all(bytes);
        return Ok(var_int_size + bytes_len);
    }
}

#[tokio::test]
async fn encode_decode() {
    let expected = String::from("Hello world!");
    let (mut client, mut server) = tokio::io::duplex(100000);
    expected.encode(&mut client).await.unwrap();
    assert_eq!(expected, String::decode(&mut server).await.unwrap())
}
