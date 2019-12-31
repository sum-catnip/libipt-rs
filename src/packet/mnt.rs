use libipt_sys::{pt_packet_mnt, pt_packet_type_ppt_mnt};

/// A MNT packet.
/// Packet: mnt
#[derive(Clone, Copy)]
pub struct Mnt (pt_packet_mnt);
impl Mnt {
    #[inline]
    pub fn new(payload: u64) -> Self { Mnt(pt_packet_mnt{payload}) }

    #[inline]
    /// The raw payload
    pub fn payload(self) -> u64 { self.0.payload }

    #[inline]
    /// The raw payload
    pub fn set_payload(&mut self, payload: u64) { self.0.payload = payload }
}

wrap2raw!(Mnt, pt_packet_type_ppt_mnt, mnt);
raw2wrap!(Mnt, Mnt, pt_packet_mnt);