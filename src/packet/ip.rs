use std::convert::TryFrom;
use num_enum::{TryFromPrimitive, IntoPrimitive};
use libipt_sys::{
    pt_packet_ip,
    pt_packet_type_ppt_tip,
    pt_packet_type_ppt_fup,
    pt_packet_type_ppt_tip_pge,
    pt_packet_type_ppt_tip_pgd,
    pt_ip_compression_pt_ipc_full,
    pt_ip_compression_pt_ipc_sext_48,
    pt_ip_compression_pt_ipc_suppressed,
    pt_ip_compression_pt_ipc_update_16,
    pt_ip_compression_pt_ipc_update_32,
    pt_ip_compression_pt_ipc_update_48
};

/// The IP compression
#[derive(Clone, Copy, Debug, TryFromPrimitive, IntoPrimitive)]
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
/// Packet: tip
#[derive(Clone, Copy, Debug)]
pub struct Tip (pt_packet_ip);
impl Tip {
    #[inline]
    pub fn new(tip: u64, compression: Compression) -> Self {
        Tip (pt_packet_ip { ip: tip, ipc: compression.into() })
    }

    /// Zero-extended payload ip
    #[inline]
    pub fn tip(self) -> u64 { self.0.ip }
    /// Zero-extended payload ip
    #[inline]
    pub fn set_tip(&mut self, ip: u64) { self.0.ip = ip }

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

/// A packet with IP payload.
/// Packet: fup
#[derive(Clone, Copy, Debug)]
pub struct Fup (pt_packet_ip);
impl Fup {
    #[inline]
    pub fn new(fup: u64, compression: Compression) -> Self {
        Fup (pt_packet_ip { ip: fup, ipc: compression.into() })
    }

    /// Zero-extended payload ip
    #[inline]
    pub fn fup(self) -> u64 { self.0.ip }
    /// Zero-extended payload ip
    #[inline]
    pub fn set_fup(&mut self, fup: u64) { self.0.ip = fup }

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

/// A packet with IP payload.
/// Packet: tip.pge
#[derive(Clone, Copy, Debug)]
pub struct TipPge (pt_packet_ip);
impl TipPge {
    #[inline]
    pub fn new(tippge: u64, compression: Compression) -> Self {
        TipPge (pt_packet_ip { ip: tippge, ipc: compression.into() })
    }

    /// Zero-extended payload ip
    #[inline]
    pub fn tippge(self) -> u64 { self.0.ip }
    /// Zero-extended payload ip
    #[inline]
    pub fn set_tippge(&mut self, tippge: u64) { self.0.ip = tippge }

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

/// A packet with IP payload.
/// Packet: tip.pgd
#[derive(Clone, Copy, Debug)]
pub struct TipPgd (pt_packet_ip);
impl TipPgd {
    #[inline]
    pub fn new(tippgd: u64, compression: Compression) -> Self {
        TipPgd (pt_packet_ip { ip: tippgd, ipc: compression.into() })
    }

    /// Zero-extended payload ip
    #[inline]
    pub fn tippgd(self) -> u64 { self.0.ip }
    /// Zero-extended payload ip
    #[inline]
    pub fn set_tippgd(&mut self, tippgd: u64) { self.0.ip = tippgd }

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

wrap2raw!(Fup, pt_packet_type_ppt_fup, ip);
raw2wrap!(Fup, Fup, pt_packet_ip);

wrap2raw!(Tip, pt_packet_type_ppt_tip, ip);
raw2wrap!(Tip, Tip, pt_packet_ip);

wrap2raw!(TipPge, pt_packet_type_ppt_tip_pge, ip);
raw2wrap!(TipPge, TipPge, pt_packet_ip);

wrap2raw!(TipPgd, pt_packet_type_ppt_tip_pgd, ip);
raw2wrap!(TipPgd, TipPgd, pt_packet_ip);
