use super::version::ProtocolVersion;

/// Trait defining a protocol translation pipeline for bidirectional packet transformation.
pub trait PacketTranslator: Send + Sync {
    /// Source (server) protocol version.
    fn source_version(&self) -> ProtocolVersion;
    
    /// Target (client - Horizon native 26.3) protocol version.
    fn target_version(&self) -> ProtocolVersion;

    /// Transforms an incoming server packet into a Horizon-compatible packet.
    fn translate_incoming(&self, packet_id: i32, payload: &[u8]) -> Option<(i32, Vec<u8>)>;

    /// Transforms an outgoing Horizon packet into a server-compatible packet.
    fn translate_outgoing(&self, packet_id: i32, payload: &[u8]) -> Option<(i32, Vec<u8>)>;
}

/// A passthrough translator used when connected to native Horizon / 26.3 servers.
pub struct IdentityTranslator {
    version: ProtocolVersion,
}

impl IdentityTranslator {
    pub fn new(version: ProtocolVersion) -> Self {
        Self { version }
    }
}

impl PacketTranslator for IdentityTranslator {
    fn source_version(&self) -> ProtocolVersion {
        self.version
    }

    fn target_version(&self) -> ProtocolVersion {
        self.version
    }

    fn translate_incoming(&self, packet_id: i32, payload: &[u8]) -> Option<(i32, Vec<u8>)> {
        Some((packet_id, payload.to_vec()))
    }

    fn translate_outgoing(&self, packet_id: i32, payload: &[u8]) -> Option<(i32, Vec<u8>)> {
        Some((packet_id, payload.to_vec()))
    }
}
