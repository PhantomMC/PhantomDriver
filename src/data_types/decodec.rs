use std::future::Future;

use tokio::io::{AsyncReadExt, AsyncWriteExt, Error};

pub trait Decodable: Sized {
    fn decode<S: AsyncReadExt + Unpin>(stream: &mut S)
        -> impl Future<Output = Result<Self, Error>>;
}

pub trait Encodable: Sized {
    fn encode<S: AsyncWriteExt + Unpin>(
        self: &Self,
        stream: &mut S,
    ) -> impl Future<Output = Result<(), Error>>;
}

pub trait FixedSizeDecodable<const N: usize>: Sized {
    fn fixed_decode<S: AsyncReadExt + Unpin>(
        stream: &mut S,
    ) -> impl Future<Output = Result<Self, Error>>;
}

pub trait FixedSizeEncodable<const N: usize>: Sized {
    fn fixed_encode<S: AsyncWriteExt + Unpin>(
        self: &Self,
        stream: &mut S,
    ) -> impl Future<Output = Result<(), Error>>;
}
