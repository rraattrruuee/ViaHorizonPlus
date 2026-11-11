//! # ViaHorizonPlus
//!
//! A multi-version protocol compatibility and legacy emulation native mod for Horizon
//! (Minecraft: Java Edition 26.3 rewrite in Rust).
//!
//! Based on the concepts and architecture of ViaFabricPlus, ported to Rust and
//! integrated using Horizon's `horizon-mixins` system.

pub mod emulation;
pub mod gui;
pub mod hooks;
pub mod protocol;
pub mod state;

pub use emulation::EmulationProfile;
pub use protocol::{HandshakePacket, ProtocolVersion};
pub use state::{global_state, ViaHorizonConfig, ViaHorizonState};

/// Initializes the ViaHorizonPlus mod runtime.
pub fn init() {
    log::info!("[ViaHorizonPlus] Initialized native mod for Horizon");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_versions() {
        assert_eq!(ProtocolVersion::Release1_8.protocol_number(), 47);
        assert_eq!(ProtocolVersion::HorizonNative.protocol_number(), 777);
        assert_eq!(ProtocolVersion::from_protocol_number(47), Some(ProtocolVersion::Release1_8));
    }

    #[test]
    fn test_handshake_rewrite() {
        let mut packet = HandshakePacket {
            protocol_version: 777,
            server_address: "hypixel.net".to_string(),
            server_port: 25565,
            next_state: protocol::handshake::HandshakeIntent::Login,
        };

        let encoded = packet.encode().expect("encode should succeed");
        let decoded = HandshakePacket::decode(&encoded).expect("decode should succeed");
        assert_eq!(packet, decoded);

        hooks::network::intercept_handshake(&mut packet);
        // Default target is HorizonNative, so protocol_version remains 777
        assert_eq!(packet.protocol_version, 777);

        // Change target to 1.8
        global_state().set_target_protocol(ProtocolVersion::Release1_8);
        hooks::network::intercept_handshake(&mut packet);
        assert_eq!(packet.protocol_version, 47);
    }
}
