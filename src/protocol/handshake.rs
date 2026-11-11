use super::version::ProtocolVersion;
use std::io::{Cursor, Read, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandshakeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid varint")]
    InvalidVarInt,
    #[error("Invalid next state: {0}")]
    InvalidNextState(i32),
}

/// Represents the Minecraft client-to-server Handshake packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandshakePacket {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: HandshakeIntent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakeIntent {
    Status = 1,
    Login = 2,
}

impl HandshakePacket {
    /// Rewrites the handshake protocol version to match the targeted ProtocolVersion.
    pub fn rewrite_for_target(&mut self, target: ProtocolVersion) {
        self.protocol_version = target.protocol_number();
    }

    /// Decodes a handshake packet from raw bytes (payload following packet ID 0x00).
    pub fn decode(bytes: &[u8]) -> Result<Self, HandshakeError> {
        let mut cursor = Cursor::new(bytes);
        let protocol_version = read_varint(&mut cursor)?;
        let addr_len = read_varint(&mut cursor)? as usize;
        let mut addr_bytes = vec![0u8; addr_len];
        cursor.read_exact(&mut addr_bytes)?;
        let server_address = String::from_utf8_lossy(&addr_bytes).to_string();

        let mut port_bytes = [0u8; 2];
        cursor.read_exact(&mut port_bytes)?;
        let server_port = u16::from_be_bytes(port_bytes);

        let next_state_raw = read_varint(&mut cursor)?;
        let next_state = match next_state_raw {
            1 => HandshakeIntent::Status,
            2 => HandshakeIntent::Login,
            other => return Err(HandshakeError::InvalidNextState(other)),
        };

        Ok(Self {
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }

    /// Encodes the handshake packet into raw bytes.
    pub fn encode(&self) -> Result<Vec<u8>, HandshakeError> {
        let mut buf = Vec::new();
        write_varint(&mut buf, self.protocol_version)?;
        write_varint(&mut buf, self.server_address.len() as i32)?;
        buf.write_all(self.server_address.as_bytes())?;
        buf.write_all(&self.server_port.to_be_bytes())?;
        write_varint(&mut buf, self.next_state as i32)?;
        Ok(buf)
    }
}

pub fn read_varint<R: Read>(reader: &mut R) -> Result<i32, HandshakeError> {
    let mut value = 0i32;
    let mut position = 0;
    let mut byte = [0u8; 1];

    loop {
        reader.read_exact(&mut byte)?;
        let current = byte[0];
        value |= ((current & 0x7F) as i32) << position;

        if (current & 0x80) == 0 {
            break;
        }

        position += 7;
        if position >= 32 {
            return Err(HandshakeError::InvalidVarInt);
        }
    }

    Ok(value)
}

pub fn write_varint<W: Write>(writer: &mut W, mut value: i32) -> Result<(), HandshakeError> {
    loop {
        if (value & !0x7F) == 0 {
            writer.write_all(&[value as u8])?;
            return Ok(());
        }

        writer.write_all(&[((value & 0x7F) | 0x80) as u8])?;
        value = ((value as u32) >> 7) as i32;
    }
}
