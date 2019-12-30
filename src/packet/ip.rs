use num_enum::{TryFromPrimitive, IntoPrimitive};
use std::convert::TryFrom;
use libipt_sys::{
    pt_packet_ip,
    pt_ip_compression_pt_ipc_full,
    pt_ip_compression_pt_ipc_sext_48,
    pt_ip_compression_pt_ipc_suppressed,
    pt_ip_compression_pt_ipc_update_16,
    pt_ip_compression_pt_ipc_update_32,
    pt_ip_compression_pt_ipc_update_48
};

/// The IP compression
#[derive(Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum Compression {
    /// No payload. The IP has been suppressed
    Suppressed = pt_ip_compression_pt_ipc_suppressed,

    /// Payload: 16 bits. Update last IP
    Update16   = pt_ip_compression_pt_ipc_update_16,

    /// Payload: 32 bits. Update last IP
    Update32   = pt_ip_compression_pt_ipc_update_32,

    /// Payload: 48 bits. Sign extend to full address
    Sext48     = pt_ip_compression_pt_ipc_sext_48,

    /// Payload: 48 bits. Update last IP
    Update48   = pt_ip_compression_pt_ipc_update_48,

    /// Payload: 64 bits. Full address
    Full       = pt_ip_compression_pt_ipc_full
}

/// A packet with IP payload.
/// Packet: tip, fup, tip.pge, tip.pgd
#[derive(Clone, Copy)]
pub struct Ip (pt_packet_ip);
impl Ip {
    #[inline]
    pub fn new(ip: u64, compression: Compression) -> Self {
        Ip (pt_packet_ip { ip, ipc: compression.into() })
    }

    #[inline]
    pub(crate) fn wrap(pck: pt_packet_ip) -> Self { Ip (pck) }

    /// Zero-extended payload ip
    #[inline]
    pub fn ip(self) -> u64 { self.0.ip }
    /// Zero-extended payload ip
    #[inline]
    pub fn set_ip(&mut self, ip: u64) { self.0.ip = ip }

    /// IP compression
    #[inline]
    pub fn compression(self) -> Compression {
        // if this tryfrom panics, there is a bug
        // in either libipt or this crate.
        Compression::try_from(self.0.ipc).unwrap()
    }

    /// IP compression
    #[inline]
    pub fn set_compression(&mut self, compression: Compression) {
        self.0.ipc = compression.into()
    }
}