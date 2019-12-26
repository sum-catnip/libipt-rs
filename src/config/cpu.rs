use libipt_sys::{
    pt_cpu,
    pt_cpu_vendor_pcv_intel,
    pt_cpu_vendor_pcv_unknown,
    pt_errata,
    pt_cpu_errata,
};

use bitflags::bitflags;

bitflags! {
    /// i suppose this is relevant when/if amd finally gets intelpt support?
    pub struct CPUVendor: i32 {
        const INTEL = pt_cpu_vendor_pcv_intel;
        const UNKNOWN = pt_cpu_vendor_pcv_unknown;
    }
}

/// a cpu identifier
#[derive(Clone, Copy)]
pub struct CPU (pub(super) pt_cpu);

impl CPU {
    pub fn new(vendor: CPUVendor, family: u16, model: u8, stepping: u8) -> Self {
        CPU(pt_cpu{ vendor: vendor.bits(), family, model, stepping })
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
