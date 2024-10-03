use crate::data_types::decodec::{Decodable, FixedSizeDecodable};

pub struct Login {
    pub player_name: String,
    pub player_uuid: Option<u128>,
}

impl Decodable for Login {
    fn decode<S: std::io::Read>(stream: &mut S) -> Result<Self, std::io::Error> {
        let player_name = String::decode(stream)?;
        let player_uuid = u128::fixed_decode(stream);
        return Ok(Login {
            player_name,
            player_uuid: player_uuid.map_or(None, Some),
        });
    }
}
