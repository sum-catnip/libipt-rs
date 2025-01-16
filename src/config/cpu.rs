// Certain casts are required only on Windows. Inform Clippy to ignore them.
#![allow(clippy::unnecessary_cast)]

use libipt_sys::{
    pt_cpu, pt_cpu_errata, pt_cpu_vendor, pt_cpu_vendor_pcv_intel, pt_cpu_vendor_pcv_unknown,
    pt_errata,
};
use std::mem;

use crate::error::ensure_ptok;
use crate::error::PtError;
use bitflags::bitflags;

bitflags! {
    /// i suppose this is relevant when/if amd finally gets intelpt support?
    #[derive(Debug)]
    pub struct CpuVendor: u32 {
        const INTEL = pt_cpu_vendor_pcv_intel as u32;
        const UNKNOWN = pt_cpu_vendor_pcv_unknown as u32;
    }
}

/// A Cpu identifier
#[derive(Clone, Copy, Debug)]
pub struct Cpu(pub(super) pt_cpu);
impl Cpu {
    pub fn new(vendor: CpuVendor, family: u16, model: u8, stepping: u8) -> Self {
        Cpu(pt_cpu {
            vendor: vendor.bits() as pt_cpu_vendor,
            family,
            model,
            stepping,
        })
    }

    /// A shortcut for creating an intel Cpu instance
    pub fn intel(family: u16, model: u8, stepping: u8) -> Self {
        Cpu::new(CpuVendor::INTEL, family, model, stepping)
    }

    /// determines processor specific workarounds
    pub(super) fn errata(self) -> Result<pt_errata, PtError> {
        let mut errata = unsafe { mem::zeroed::<pt_errata>() };
        ensure_ptok(unsafe { pt_cpu_errata(&mut errata, &self.0) })?;
        Ok(errata)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cpu_intel_shortcut() {
        let cpu1 = Cpu::intel(66, 12, 255);
        let cpu2 = Cpu::new(CpuVendor::INTEL, 66, 12, 255);
        assert_eq!(cpu1.0.vendor, cpu2.0.vendor);
        assert_eq!(cpu1.0.family, cpu2.0.family);
        assert_eq!(cpu1.0.model, cpu2.0.model);
        assert_eq!(cpu1.0.stepping, cpu2.0.stepping);
    }

    #[test]
    fn test_cpu_errata() {
        let cpu = Cpu::intel(0x6, 0x56, 11);
        let e = cpu.errata().unwrap();
        assert_eq!(e.bdm70(), 1);
        assert_eq!(e.bdm64(), 1);
        assert_eq!(e.skd007(), 0);
        assert_eq!(e.skd022(), 0);

        let cpu = Cpu::intel(0x6, 0x9e, 11);
        let e = cpu.errata().unwrap();
        assert_eq!(e.bdm64(), 0);
        assert_eq!(e.bdm70(), 1);
        assert_eq!(e.skd007(), 1);
        assert_eq!(e.skd022(), 1);
    }
}
