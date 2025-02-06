use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x00, "login/client/minecraft:hello")]
pub struct LoginStartPacket {
    pub name: String,
    pub player_uuid: Uuid,
}
