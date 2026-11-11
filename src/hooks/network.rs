use crate::protocol::handshake::HandshakePacket;
use crate::protocol::ProtocolVersion;
use crate::state::global_state;

/// Intercepts the outgoing handshake packet when connecting to a remote server.
pub fn intercept_handshake(packet: &mut HandshakePacket) {
    let state = global_state();
    let target = state.get_target_protocol();
    if target != ProtocolVersion::HorizonNative {
        log::info!(
            "[ViaHorizonPlus] Rewriting handshake protocol from {} to {} ({})",
            packet.protocol_version,
            target.protocol_number(),
            target.name()
        );
        packet.rewrite_for_target(target);
    }
}
