use libipt_sys::pt_packet_mtc;

/// A MTC packet.
/// Packet: mtc
#[derive(Clone, Copy)]
pub struct Mtc (pt_packet_mtc);
impl Mtc {
    #[inline]
    pub fn new(ctc: u8) -> Self { Mtc(pt_packet_mtc{ctc}) }

    #[inline]
    pub(crate) fn wrap(pck: pt_packet_mtc) -> Self { Mtc(pck) }

    #[inline]
    /// The crystal clock tick counter value
    pub fn ctc(self) -> u8 { self.0.ctc }

    #[inline]
    /// The crystal clock tick counter value
    pub fn set_ctc(&mut self, ctc: u8) { self.0.ctc = ctc }
}