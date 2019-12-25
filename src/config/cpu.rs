use libipt_sys::{
    pt_cpu,
    pt_cpu_vendor_pcv_intel,
    pt_cpu_vendor_pcv_unknown,
    pt_errata,
    pt_cpu_errata,
};

use bitflags::bitflags;

bitflags! {
    /// i suppose this is relevant when/if amd finally gets intelpt support
    #[derive(Default)]
    pub struct CPUVendor: i32 {
        const INTEL = pt_cpu_vendor_pcv_intel;
        const UNKNOWN = pt_cpu_vendor_pcv_unknown;
    }
}

/// a cpu identifier
#[derive(Clone, Copy, Default)]
pub struct CPU {
    vendor:   CPUVendor,
    family:   u16,
    model:    u8,
    stepping: u8
}

impl From<CPU> for pt_cpu {
    fn from(cpu: CPU) -> Self {
        pt_cpu{
            vendor:   cpu.vendor.bits(),
            family:   cpu.family,
            model:    cpu.model,
            stepping: cpu.stepping
        }
    }
}

impl CPU {
    pub fn new(vendor: CPUVendor, family: u16, model: u8, stepping: u8) -> Self {
        CPU {vendor, family, model, stepping}
    }

    /// determines processor specific workarounds
    pub fn determine_errata(self) -> pt_errata {
        let mut errata = pt_errata {
            _bitfield_1: Default::default(),
            reserved: Default::default()
        };
        // we dont care about errors here since
        // itll just return an empty errata
        unsafe{ pt_cpu_errata(&mut errata, &(self.into())); }
        errata
    }
}
