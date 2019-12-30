use libipt_sys::pt_packet_tsc;

/// A TSC packet.
/// Packet: tsc
#[derive(Clone, Copy)]
pub struct Tsc (pt_packet_tsc);
impl Tsc {
    #[inline]
    pub fn new(tsc: u64) -> Self { Tsc(pt_packet_tsc{tsc}) }

    #[inline]
    pub(crate) fn wrap(pck: pt_packet_tsc) -> Self { Tsc(pck) }

    #[inline]
    /// The TSC value
    pub fn tsc(self) -> u64 { self.0.tsc }

    #[inline]
    /// The TSC value
    pub fn set_tsc(&mut self, tsc: u64) { self.0.tsc = tsc }
}