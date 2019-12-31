use libipt_sys::{pt_packet_tma, pt_packet_type_ppt_tma};

/// A TMA packet.
/// Packet: tma
#[derive(Clone, Copy)]
pub struct Tma (pt_packet_tma);
impl Tma {
    #[inline]
    pub fn new(ctc: u16, fc: u16) -> Self { Tma(pt_packet_tma{ctc, fc}) }

    #[inline]
    /// The crystal clock tick counter value
    pub fn ctc(self) -> u16 { self.0.ctc }

    #[inline]
    /// The crystal clock tick counter value
    pub fn set_ctc(&mut self, ctc: u16) { self.0.ctc = ctc }

    #[inline]
    /// The fast counter value
    pub fn fc(self) -> u16 { self.0.fc }

    #[inline]
    /// The fast counter value
    pub fn set_fc(&mut self, fc: u16) { self.0.fc = fc }
}

wrap2raw!(Tma, pt_packet_type_ppt_tma, tma);
raw2wrap!(Tma, Tma, pt_packet_tma);