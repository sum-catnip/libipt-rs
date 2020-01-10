use std::mem;
use std::convert::TryFrom;
use libipt_sys::pt_conf_addr_filter;
use num_enum::TryFromPrimitive;


#[derive(Clone, Copy, TryFromPrimitive)]
#[repr(u32)]
pub enum AddrConfig {DISABLED, FILTER, STOP }

/// an address range inside the address filter
#[derive(Clone, Copy)]
pub struct AddrRange {
    /// This corresponds to the IA32_RTIT_ADDRn_A MSRs
    a: u64,
    /// This corresponds to the IA32_RTIT_ADDRn_B MSRs
    b: u64,
    /// this corresponds to the respective fields in IA32_RTIT_CTL MSR
    cfg: AddrConfig
}

impl AddrRange {
    #[inline]
    pub fn new(a: u64, b: u64, cfg: AddrConfig) -> Self {
        AddrRange { a, b, cfg }
    }

    /// This corresponds to the IA32_RTIT_ADDRn_A MSRs
    #[inline]
    pub fn a(&self) -> u64 { self.a }
    /// This corresponds to the IA32_RTIT_ADDRn_B MSRs
    #[inline]
    pub fn b(&self) -> u64 { self.b }
    /// this corresponds to the respective fields in IA32_RTIT_CTL MSR
    #[inline]
    pub fn cfg(&self) -> AddrConfig { self.cfg }
    
    /// This corresponds to the IA32_RTIT_ADDRn_A MSRs
    #[inline]
    pub fn set_a(&mut self, a: u64) { self.a = a; }
    /// This corresponds to the IA32_RTIT_ADDRn_B MSRs
    #[inline]
    pub fn set_b(&mut self, b: u64) { self.b = b; }
    /// this corresponds to the respective fields in IA32_RTIT_CTL MSR
    #[inline]
    pub fn set_cfg(&mut self, cfg: AddrConfig) { self.cfg = cfg }
}

// could've written a macro, i know
// but its just like 4 variables i think its fine

/// the address filter configuration
#[derive(Clone, Copy)]
pub struct AddrFilter (pub(super) pt_conf_addr_filter);
impl AddrFilter {
    pub fn new() -> Self { unsafe { mem::zeroed() }}

    #[inline]
    pub fn set_addr0(&mut self, range: AddrRange) {
        self.0.addr0_a = range.a;
        self.0.addr0_b = range.b;
        unsafe { self.0.config.ctl.set_addr0_cfg(range.cfg as u32) };
    }

    #[inline]
    pub fn set_addr1(&mut self, range: AddrRange) {
        self.0.addr1_a = range.a;
        self.0.addr1_b = range.b;
        unsafe { self.0.config.ctl.set_addr1_cfg(range.cfg as u32) };
    }

    #[inline]
    pub fn set_addr2(&mut self, range: AddrRange) {
        self.0.addr2_a = range.a;
        self.0.addr2_b = range.b;
        unsafe { self.0.config.ctl.set_addr2_cfg(range.cfg as u32) };
    }

    #[inline]
    pub fn set_addr3(&mut self, range: AddrRange) {
        self.0.addr3_a = range.a;
        self.0.addr3_b = range.b;
        unsafe { self.0.config.ctl.set_addr3_cfg(range.cfg as u32) };
    }

    #[inline]
    pub fn addr0(&self) -> AddrRange {
        unsafe {
            AddrRange::new(self.0.addr0_a, self.0.addr0_b,
                AddrConfig::try_from(self.0.config.ctl.addr0_cfg()).unwrap())
        }
    }

    #[inline]
    pub fn addr1(&self) -> AddrRange {
        unsafe {
            AddrRange::new(self.0.addr1_a, self.0.addr1_b,
                AddrConfig::try_from(self.0.config.ctl.addr1_cfg()).unwrap())
        }
    }

    #[inline]
    pub fn addr2(&self) -> AddrRange {
        unsafe {
            AddrRange::new(self.0.addr2_a, self.0.addr2_b,
                AddrConfig::try_from(self.0.config.ctl.addr2_cfg()).unwrap())
        }
    }

    #[inline]
    pub fn addr3(&self) -> AddrRange {
        unsafe {
            AddrRange::new(self.0.addr3_a, self.0.addr3_b,
                AddrConfig::try_from(self.0.config.ctl.addr3_cfg()).unwrap())
        }
    }
}