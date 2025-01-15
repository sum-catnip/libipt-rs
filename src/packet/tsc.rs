use libipt_sys::{pt_packet_tsc, pt_packet_type_ppt_tsc};

/// A TSC packet.
/// Packet: tsc
#[derive(Clone, Copy, Debug)]
pub struct Tsc(pt_packet_tsc);
impl Tsc {
    #[inline]
    #[must_use]
    pub fn new(tsc: u64) -> Self {
        Tsc(pt_packet_tsc { tsc })
    }

    /// The TSC value
    #[inline]
    #[must_use]
    pub fn tsc(&self) -> u64 {
        self.0.tsc
    }

    #[inline]
    /// The TSC value
    pub fn set_tsc(&mut self, tsc: u64) {
        self.0.tsc = tsc;
    }
}

wrap2raw!(Tsc, pt_packet_type_ppt_tsc, tsc);
raw2wrap!(Tsc, Tsc, pt_packet_tsc);
