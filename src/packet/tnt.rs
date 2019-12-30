use libipt_sys::pt_packet_tnt;

/// A TNT-8 or TNT-64 packet.
/// Packet: tnt-8, tnt-64
#[derive(Clone, Copy)]
pub struct Tnt (pt_packet_tnt);
impl Tnt {
    #[inline]
    pub fn new(payload: u64, bitsize: u8) -> Self {
        Tnt (pt_packet_tnt {bit_size: bitsize, payload})
    }

    #[inline]
    pub(crate) fn wrap(pck: pt_packet_tnt) -> Self {
        Tnt (pck)
    }

    /// TNT payload bit size
    #[inline]
    pub fn bitsize(self) -> u8 { self.0.bit_size }
    /// TNT payload bit size
    #[inline]
    pub fn set_bitsize(&mut self, sz: u8) { self.0.bit_size = sz }
    /// TNT payload excluding stop bit
    #[inline]
    pub fn payload(self) -> u64 { self.0.payload }
    /// TNT payload excluding stop bit
    #[inline]
    pub fn set_payload(&mut self, payload: u64) {
        self.0.payload = payload
    }
}