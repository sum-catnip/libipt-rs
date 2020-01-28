use libipt_sys::{
    pt_cpu,
    pt_cpu_vendor_pcv_intel,
    pt_cpu_vendor_pcv_unknown,
    pt_errata,
    pt_cpu_errata,
};

use bitflags::bitflags;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cpu_intel_shortcut() {
        let cpu1 = Cpu::intel(66, 12, 255);
        let cpu2 = Cpu::new(CpuVendor::INTEL, 66, 12, 255);
        assert_eq!(cpu1.0.vendor, cpu2.0.vendor);
        assert_eq!(cpu1.0.family, cpu2.0.family);
        assert_eq!(cpu1.0.model,  cpu2.0.model);
        assert_eq!(cpu1.0.stepping, cpu2.0.stepping);
    }

    #[test]
    fn test_cpu_errata() {
        let cpu = Cpu::intel(0x6, 0x56, 11);
        let e = cpu.determine_errata();
        assert_eq!(e.bdm70(), 1);
        assert_eq!(e.bdm64(), 1);
        assert_eq!(e.skd007(), 0);
        assert_eq!(e.skd022(), 0);
        
        let cpu = Cpu::intel(0x6, 0x9e, 11);
        let e = cpu.determine_errata();
        assert_eq!(e.bdm64(), 0);
        assert_eq!(e.bdm70(), 1);
        assert_eq!(e.skd007(), 1);
        assert_eq!(e.skd022(), 1);
    }
}

bitflags! {
    /// i suppose this is relevant when/if amd finally gets intelpt support?
    pub struct CpuVendor: i32 {
        const INTEL = pt_cpu_vendor_pcv_intel;
        const UNKNOWN = pt_cpu_vendor_pcv_unknown;
    }
}

/// A Cpu identifier
#[derive(Clone, Copy, Debug)]
pub struct Cpu (pub(super) pt_cpu);
impl Cpu {
    pub fn new(vendor: CpuVendor, family: u16, model: u8, stepping: u8) -> Self {
        Cpu(pt_cpu{ vendor: vendor.bits(), family, model, stepping })
    }

    /// A shortcut for creating an intel Cpu instance
    pub fn intel(family: u16, model: u8, stepping: u8) -> Self {
        Cpu::new(CpuVendor::INTEL, family, model, stepping)
    }

    /// determines processor specific workarounds
    pub(super) fn determine_errata(self) -> pt_errata {
        let mut errata = pt_errata {
            _bitfield_1: Default::default(),
            reserved: Default::default()
        };
        // we dont care about errors here since
        // itll just return an empty errata
        unsafe{ pt_cpu_errata(&mut errata, &self.0); }
        errata
    }
}
