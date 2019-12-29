use bitflags::bitflags;
use num_enum::TryFromPrimitive;
use libipt_sys::{
    pt_ip_compression_pt_ipc_full,
    pt_ip_compression_pt_ipc_sext_48,
    pt_ip_compression_pt_ipc_suppressed,
    pt_ip_compression_pt_ipc_update_16,
    pt_ip_compression_pt_ipc_update_32,
    pt_ip_compression_pt_ipc_update_48,

    pt_mode_leaf_pt_mol_exec,
    pt_mode_leaf_pt_mol_tsx
};

/// A TNT-8 or TNT-64 packet
/// Packet: tnt-8, tnt-64
#[derive(Clone, Copy)]
pub struct Tnt {
    /// TNT payload bit size
    bitSize: u8,
    /// TNT payload excluding stop bit
    payload: u64
}

/// The IP compression
#[derive(Clone, Copy, TryFromPrimitive)]
#[repr(i32)]
pub enum IpCompression {
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

/// A packet with IP payload
/// Packet: tip, fup, tip.pge, tip.pgd
#[derive(Clone, Copy)]
pub struct Ip {
    /// IP compression
    compression: IpCompression,
    /// Zero-extended payload ip
    ip: u64
}

/// Mode packet leaves
#[derive(Clone, Copy, TryFromPrimitive)]
#[repr(i32)]
pub enum ModeLeaf {
    Exec = pt_mode_leaf_pt_mol_exec,
    Tsx  = pt_mode_leaf_pt_mol_tsx
}

bitflags! {
    /// A mode.exec packet
    #[derive(Clone, Copy)]
    pub struct ModeExec : u32 {
        /// The mode.exec csl bit
        const CSL = 0b00000001,
        /// The mode.exec csd bit
        const CSD = 0b00000010
    }
}

bitflags! {
    /// A mode.tsx packet
    #[derive(Clone, Copy)]
    pub struct ModeTsx : u32 {
        /// The mode.tsx intx bit
        const intx = 0b00000001,
        /// The mode.tsx abrt bit
        const abrt = 0b00000010
    }
}

pub enum ModeBits {
    Exec(ModeExec),
    Tsx(ModeTsx)
}

/// A mode packet
/// Packet: mode
#[derive(Clone, Copy)]
pub struct Mode {
    /// Mode leaf
    leaf: ModeLeaf,
    /// Mode bits
    bits: ModeBits
}