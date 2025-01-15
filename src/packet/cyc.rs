use libipt_sys::{pt_packet_cyc, pt_packet_type_ppt_cyc};

/// A CYC packet.
/// Packet: cyc
#[derive(Clone, Copy, Debug)]
pub struct Cyc(pt_packet_cyc);
impl Cyc {
    #[inline]
    #[must_use]
    pub fn new(value: u64) -> Self {
        Cyc(pt_packet_cyc { value })
    }

    /// The cycle counter value
    #[inline]
    #[must_use]
    pub fn value(&self) -> u64 {
        self.0.value
    }

    #[inline]
    /// The cycle counter value
    pub fn set_value(&mut self, value: u64) {
        self.0.value = value;
    }
}

wrap2raw!(Cyc, pt_packet_type_ppt_cyc, cyc);
raw2wrap!(Cyc, Cyc, pt_packet_cyc);
