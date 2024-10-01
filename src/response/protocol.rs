use crate::{data_types::decodec::Encodable, request::slp_history::SlpHistory};

pub trait ProtocolResponse {
    fn respond<E: Encodable>(self: &Self, history: SlpHistory) -> E;
}
