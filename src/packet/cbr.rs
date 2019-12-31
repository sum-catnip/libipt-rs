use libipt_sys::{pt_packet_cbr, pt_packet_type_ppt_cbr};

/// A CBR packet.
/// Packet: cbr
#[derive(Clone, Copy)]
pub struct Cbr (pt_packet_cbr);
impl Cbr {
    #[inline]
    pub fn new(ratio: u8) -> Self { Cbr(pt_packet_cbr{ratio}) }

    #[inline]
    /// The core/bus cycle ratio
    pub fn ratio(self) -> u8 { self.0.ratio }

    #[inline]
    /// The core/bus cycle ratio
    pub fn set_ratio(&mut self, ratio: u8) { self.0.ratio = ratio }
}

wrap2raw!(Cbr, pt_packet_type_ppt_cbr, cbr);
raw2wrap!(Cbr, Cbr, pt_packet_cbr);