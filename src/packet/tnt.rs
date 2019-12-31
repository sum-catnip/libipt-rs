use libipt_sys::{
    pt_packet_tnt,
    pt_packet_type_ppt_tnt_8,
    pt_packet_type_ppt_tnt_64
};

/// A TNT-8 packet.
/// Packet: tnt-8
#[derive(Clone, Copy)]
pub struct Tnt8 (pt_packet_tnt);
impl Tnt8 {
    #[inline]
    pub fn new(payload: u8, bitsize: u8) -> Self {
        Tnt8 (pt_packet_tnt {bit_size: bitsize, payload: payload as u64})
    }

    /// TNT payload bit size
    #[inline]
    pub fn bitsize(self) -> u8 { self.0.bit_size }
    /// TNT payload bit size
    #[inline]
    pub fn set_bitsize(&mut self, sz: u8) { self.0.bit_size = sz }
    /// TNT payload excluding stop bit
    #[inline]
    pub fn payload(self) -> u8 { self.0.payload as u8 }
    /// TNT payload excluding stop bit
    #[inline]
    pub fn set_payload(&mut self, payload: u8) {
        self.0.payload = payload as u64
    }
}

/// A TNT-64 packet.
/// Packet: tnt-64
#[derive(Clone, Copy)]
pub struct Tnt64 (pt_packet_tnt);
impl Tnt64 {
    #[inline]
    pub fn new(payload: u64, bitsize: u8) -> Self {
        Tnt64 (pt_packet_tnt {bit_size: bitsize, payload})
    }

    /// TNT payload bit size
    #[inline]
    pub fn bitsize(self) -> u8 { self.0.bit_size }
    /// TNT payload bit size
    #[inline]
    pub fn set_bitsize(&mut self, sz: u8) { self.0.bit_size = sz }
    /// TNT payload excluding stop bit
    #[inline]
    pub fn payload(self) -> u8 { self.0.payload as u8 }
    /// TNT payload excluding stop bit
    #[inline]
    pub fn set_payload(&mut self, payload: u8) {
        self.0.payload = payload as u64
    }
}

wrap2raw!(Tnt8, pt_packet_type_ppt_tnt_8, tnt);
raw2wrap!(Tnt8, Tnt8, pt_packet_tnt);

wrap2raw!(Tnt64, pt_packet_type_ppt_tnt_64, tnt);
raw2wrap!(Tnt64, Tnt64, pt_packet_tnt);