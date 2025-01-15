use libipt_sys::{pt_packet_mtc, pt_packet_type_ppt_mtc};

/// A MTC packet.
/// Packet: mtc
#[derive(Clone, Copy, Debug)]
pub struct Mtc(pt_packet_mtc);
impl Mtc {
    #[inline]
    #[must_use]
    pub fn new(ctc: u8) -> Self {
        Mtc(pt_packet_mtc { ctc })
    }

    /// The crystal clock tick counter value
    #[inline]
    #[must_use]
    pub fn ctc(&self) -> u8 {
        self.0.ctc
    }

    #[inline]
    /// The crystal clock tick counter value
    pub fn set_ctc(&mut self, ctc: u8) {
        self.0.ctc = ctc;
    }
}

wrap2raw!(Mtc, pt_packet_type_ppt_mtc, mtc);
raw2wrap!(Mtc, Mtc, pt_packet_mtc);
