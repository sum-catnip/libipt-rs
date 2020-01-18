use libipt_sys::{pt_packet_vmcs, pt_packet_type_ppt_vmcs};

/// A VMCS packet.
/// Packet: vmcs
#[derive(Clone, Copy, Debug)]
pub struct Vmcs (pt_packet_vmcs);
impl Vmcs {
    #[inline]
    pub fn new(base: u64) -> Self { Vmcs(pt_packet_vmcs{base}) }

    #[inline]
    /// The VMCS Base Address (i.e. the shifted payload)
    pub fn base(self) -> u64 { self.0.base }

    #[inline]
    /// The VMCS Base Address (i.e. the shifted payload)
    pub fn set_base(&mut self, base: u64) { self.0.base = base }
}

wrap2raw!(Vmcs, pt_packet_type_ppt_vmcs, vmcs);
raw2wrap!(Vmcs, Vmcs, pt_packet_vmcs);