use std::mem;
use libipt_sys::pt_asid;

#[derive(Clone, Copy)]
pub struct Asid(pub(crate) pt_asid);
impl Asid {
    #[inline]
    pub fn new(cr3: u64, vmcs: u64) -> Self {
        Asid(pt_asid {
            size: mem::size_of::<pt_asid>(),
            cr3, vmcs
        })
    }

    #[inline]
    pub fn cr3(self) -> u64 { self.0.cr3 }
    #[inline]
    pub fn set_cr3(&mut self, cr3: u64) { self.0.cr3 = cr3 }

    #[inline]
    pub fn vmcs(self) -> u64 { self.0.vmcs }
    #[inline]
    pub fn set_vmcs(&mut self, vmcs: u64) { self.0.vmcs = vmcs }
}

impl From<pt_asid> for Asid {
    fn from(asid: pt_asid) -> Self { Asid(asid) }
}