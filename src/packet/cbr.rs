use libipt_sys::pt_packet_cbr;

/// A CBR packet.
/// Packet: cbr
#[derive(Clone, Copy)]
pub struct Cbr (pt_packet_cbr);
impl Cbr {
    #[inline]
    pub fn new(ratio: u8) -> Self { Cbr(pt_packet_cbr{ratio}) }

    #[inline]
    pub(crate) fn wrap(pck: pt_packet_cbr) -> Self { Cbr(pck) }

    #[inline]
    /// The core/bus cycle ratio
    pub fn ratio(self) -> u8 { self.0.ratio }

    #[inline]
    /// The core/bus cycle ratio
    pub fn set_ratio(&mut self, ratio: u8) { self.0.ratio = ratio }
}