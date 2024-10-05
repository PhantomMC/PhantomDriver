use tokio::io::{AsyncReadExt, AsyncWriteExt, Error};

pub trait Decodable: Sized {
    async fn decode<S: AsyncReadExt + Unpin>(stream: &mut S) -> Result<Self, Error>;
}

pub trait Encodable: Sized {
    async fn encode<S: AsyncWriteExt + Unpin>(
        self: &Self,
        stream: &mut S,
    ) -> Result<(usize), Error>;
}

pub trait FixedSizeDecodable<const N: usize>: Sized {
    async fn fixed_decode<S: AsyncReadExt + Unpin>(stream: &mut S) -> Result<Self, Error>;
}

pub trait FixedSizeEncodable<const N: usize>: Sized {
    async fn fixed_encode<S: AsyncWriteExt + Unpin>(
        self: &Self,
        stream: &mut S,
    ) -> Result<(), Error>;
}
