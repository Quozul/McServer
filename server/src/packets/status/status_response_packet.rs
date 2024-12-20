use crate::packets::status::status_response::StatusResponse;
use protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id(0x00)]
pub struct StatusResponsePacket {
    json_response: String,
}

impl StatusResponsePacket {
    pub fn from_status_response(status_response: &StatusResponse) -> Self {
        let json_response = serde_json::to_string(status_response).unwrap();
        StatusResponsePacket { json_response }
    }
}
