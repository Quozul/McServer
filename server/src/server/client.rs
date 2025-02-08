use crate::network::packet_stream::{PacketStream, PacketStreamError};
use crate::network::raw_packet::RawPacket;
use crate::packets::play::client_bound_keep_alive_packet::ClientBoundKeepAlivePacket;
use crate::server::game_profile::GameProfile;
use crate::server::packet_map::{PacketMap, PacketRecipient};
use crate::server::protocol_version::ProtocolVersion;
use crate::server::server::NamedPacket;
use crate::state::State;
use protocol::prelude::{EncodePacket, PacketId};
use rand::Rng;
use std::sync::Arc;
use thiserror::Error;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tracing::{debug, error};

pub struct Client {
    state: State,
    packet_reader: PacketStream<TcpStream>,
    packet_map: PacketMap,
    game_profile: Option<GameProfile>,
    version: Option<ProtocolVersion>,
}

#[derive(Debug, Error)]
pub enum ClientReadPacketError {
    #[error(transparent)]
    PacketStream(#[from] PacketStreamError),
    #[error("unknown packet {0}")]
    UnknownPacket(u8),
}

impl Client {
    pub fn new(socket: TcpStream, packet_map: PacketMap) -> Self {
        let packet_reader = PacketStream::new(socket);
        Self {
            packet_reader,
            packet_map,
            state: State::default(),
            game_profile: None,
            version: None,
        }
    }

    pub async fn read_packet(&mut self) -> Result<NamedPacket, ClientReadPacketError> {
        let packet = self.packet_reader.read_packet().await?;
        let packet_id = packet.packet_id();
        if let Some(packet_name) = self.get_packet_name_from_id(packet_id) {
            debug!("received packet {} (id={})", packet_name, packet_id);
            Ok(NamedPacket {
                name: packet_name,
                data: packet.data().to_vec(),
            })
        } else {
            Err(ClientReadPacketError::UnknownPacket(packet_id))
        }
    }

    pub fn update_state(&mut self, new_state: State) {
        debug!("update state: {}", new_state);
        self.state = new_state;
    }

    pub async fn send_packet(&mut self, packet: impl EncodePacket + PacketId) {
        let version = self.version.clone().unwrap_or_default();
        let result: anyhow::Result<()> = async {
            let packet_id = self
                .packet_map
                .get_packet_id(version, packet.get_packet_name())
                .ok();

            if let Some(packet_id) = packet_id {
                debug!(
                    "sending packet {} (id={})",
                    packet.get_packet_name(),
                    packet_id
                );

                let raw_packet = RawPacket::from_packet(packet_id, packet)?;
                self.packet_reader.write_packet(raw_packet).await?;
                Ok(())
            } else {
                error!("Unknown packet {}", packet.get_packet_name());
                Err(anyhow::anyhow!("No packet found"))
            }
        }
        .await;

        if let Err(err) = result {
            error!("error sending packet: {:?}", err);
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub async fn send_keep_alive(&mut self) {
        // Send Keep Alive
        if self.state() == &State::Play {
            let packet = ClientBoundKeepAlivePacket::new(get_random());
            self.send_packet(packet).await;
        }
    }

    pub fn set_game_profile(&mut self, profile: GameProfile) {
        self.game_profile = Some(profile);
    }

    pub fn set_protocol(&mut self, protocol_version: ProtocolVersion) {
        self.version = Some(protocol_version);
    }

    pub fn protocol_version(&self) -> ProtocolVersion {
        self.version.clone().unwrap_or_default()
    }

    fn get_packet_name_from_id(&self, packet_id: u8) -> Option<String> {
        self.packet_map
            .get_packet_name(
                self.version.clone().unwrap_or_default(),
                self.state.clone(),
                PacketRecipient::Server,
                packet_id,
            )
            .unwrap_or_else(|err| {
                error!("error getting packet name: {:?}", err);
                None
            })
    }
}

fn get_random() -> i64 {
    let mut rng = rand::rng();
    rng.random()
}

pub type SharedClient = Arc<Mutex<Client>>;
