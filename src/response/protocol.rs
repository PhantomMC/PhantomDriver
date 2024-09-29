pub trait ProtocolResponse {
    fn handle() -> Vec<u8>;
    fn protocol_version() -> u16;
}
