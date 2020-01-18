use libipt_sys::{pt_packet_mwait, pt_packet_type_ppt_mwait};

/// A MWAIT packet.
/// Packet: mwait
#[derive(Clone, Copy, Debug)]
pub struct Mwait (pt_packet_mwait);
impl Mwait {
    #[inline]
    pub fn new(ext: u32, hints: u32) -> Self {
        Mwait(pt_packet_mwait{ext, hints})
    }

    /// The MWAIT extensions (ECX)
    #[inline]
    pub fn ext(self) -> u32 { self.0.ext }

    /// The MWAIT extensions (ECX)
    #[inline]
    pub fn set_ext(&mut self, ext: u32) { self.0.ext = ext }

    /// The MWAIT hints (EAX)
    #[inline]
    pub fn hints(self) -> u32 { self.0.hints }

    /// The MWAIT hints (EAX)
    #[inline]
    pub fn set_hints(&mut self, hints: u32) { self.0.hints = hints }
}

wrap2raw!(Mwait, pt_packet_type_ppt_mwait, mwait);
raw2wrap!(Mwait, Mwait, pt_packet_mwait);