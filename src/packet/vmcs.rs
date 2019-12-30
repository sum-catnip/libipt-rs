use libipt_sys::pt_packet_vmcs;

/// A VMCS packet.
/// Packet: vmcs
#[derive(Clone, Copy)]
pub struct Vmcs (pt_packet_vmcs);
impl Vmcs {
    #[inline]
    pub fn new(base: u64) -> Self { Vmcs(pt_packet_vmcs{base}) }

    #[inline]
    pub(crate) fn wrap(pck: pt_packet_vmcs) -> Self { Vmcs(pck) }

    #[inline]
    /// The VMCS Base Address (i.e. the shifted payload)
    pub fn base(self) -> u64 { self.0.base }

    #[inline]
    /// The VMCS Base Address (i.e. the shifted payload)
    pub fn set_base(&mut self, base: u64) { self.0.base = base }
}