use bytes::{Buf, BufMut, BytesMut};
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use std::io::{Read, Write};
use tracing::warn;

const MAGIC: u16 = 7937;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Peer {
    Server,
    Client,
}

#[derive(Debug, Clone)]
pub struct PacketInfo {
    pub _sequence: u64,
    pub _peer: Peer,
    pub msg_id: u32,
    pub body: Vec<u8>,
    pub _checksum: [u8; 16],
}

pub struct PacketParser {
    buf: BytesMut,
}

impl PacketParser {
    pub fn new() -> Self {
        Self { buf: BytesMut::with_capacity(4096) }
    }

    pub fn add_raw(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);
    }

    pub fn parse_packet(&mut self) -> Option<PacketInfo> {
        if self.buf.len() < 39 {
            return None;
        }
        let body_size = u32::from_le_bytes([self.buf[10], self.buf[11], self.buf[12], self.buf[13]]) as usize;
        let total_size = 39 + body_size;
        if self.buf.len() < total_size {
            return None;
        }

        let mut packet = self.buf.split_to(total_size);
        let magic = packet.get_u16_le();
        if magic != MAGIC {
            warn!("Network: Base magic code: {} => {}", MAGIC, magic);
        }
        let msg_id = packet.get_u32_le();
        let _reserved = packet.get_u32_le();
        let _body_size = packet.get_u32_le();
        let flags = packet.get_u8();
        let sequence = packet.get_u64_le();
        let mut checksum = [0u8; 16];
        packet.copy_to_slice(&mut checksum);
        let mut body = packet.to_vec();

        if flags & 0b10 != 0 {
            body = zlib_decompress(&body).unwrap_or_else(|e| {
                warn!("zlib decompress failed: {}", e);
                body
            });
        }
        let peer = if flags & 0b1 != 0 { Peer::Client } else { Peer::Server };

        Some(PacketInfo {
            _sequence: sequence,
            _peer: peer,
            msg_id,
            body,
            _checksum: checksum,
        })
    }
}

pub struct PacketBuilder;

impl PacketBuilder {
    pub fn build_packet(msg_id: u32, body: &[u8], sequence: u64, compress: bool, peer: Peer) -> Vec<u8> {
        let mut flags: u8 = 0;
        if peer == Peer::Client {
            flags |= 1 << 0;
        }
        let body_bytes = if compress {
            flags |= 1 << 1;
            zlib_compress(body).unwrap_or_else(|_| body.to_vec())
        } else {
            body.to_vec()
        };
        let mut buf = Vec::with_capacity(39 + body_bytes.len());

        buf.put_u16_le(MAGIC);
        buf.put_u32_le(msg_id);
        buf.put_u32_le(0);
        buf.put_u32_le(body_bytes.len() as u32);
        buf.put_u8(flags);
        buf.put_u64_le(sequence);
        buf.put_slice(&[0u8; 16]);
        buf.put_slice(&body_bytes);

        buf
    }

    pub fn build(msg_id: u32, body: &[u8], sequence: u64) -> Vec<u8> {
        Self::build_packet(msg_id, body, sequence, true, Peer::Server)
    }
}

fn zlib_compress(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    encoder.finish()
}

fn zlib_decompress(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(data);
    let mut out = Vec::new();
    decoder.read_to_end(&mut out)?;
    Ok(out)
}
