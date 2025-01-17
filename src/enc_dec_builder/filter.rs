use libipt_sys::pt_conf_addr_filter;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::mem;

/// This corresponds to the IA32_RTIT_ADDRn_CFG MSRs
#[derive(Clone, Copy, TryFromPrimitive, PartialEq, Debug)]
#[repr(u32)]
pub enum AddrConfig {
    DISABLED = 0,
    FILTER = 1,
    STOP = 2,
}

/// an address range inside the address filter
#[derive(Debug, Clone, Copy)]
pub struct AddrRange {
    /// This corresponds to the IA32_RTIT_ADDRn_A MSRs
    a: u64,
    /// This corresponds to the IA32_RTIT_ADDRn_B MSRs
    b: u64,
    /// this corresponds to the respective fields in IA32_RTIT_CTL MSR
    cfg: AddrConfig,
}

impl AddrRange {
    #[inline]
    pub const fn new(a: u64, b: u64, cfg: AddrConfig) -> Self {
        AddrRange { a, b, cfg }
    }

    /// This corresponds to the IA32_RTIT_ADDRn_A MSRs
    #[inline]
    pub fn a(&self) -> u64 {
        self.a
    }
    /// This corresponds to the IA32_RTIT_ADDRn_B MSRs
    #[inline]
    pub fn b(&self) -> u64 {
        self.b
    }
    /// this corresponds to the respective fields in IA32_RTIT_CTL MSR
    #[inline]
    pub fn cfg(&self) -> AddrConfig {
        self.cfg
    }

    /// This corresponds to the IA32_RTIT_ADDRn_A MSRs
    #[inline]
    pub const fn set_a(&mut self, a: u64) {
        self.a = a;
    }
    /// This corresponds to the IA32_RTIT_ADDRn_B MSRs
    #[inline]
    pub const fn set_b(&mut self, b: u64) {
        self.b = b;
    }
    /// this corresponds to the respective fields in IA32_RTIT_CTL MSR
    #[inline]
    pub const fn set_cfg(&mut self, cfg: AddrConfig) {
        self.cfg = cfg
    }
}

/// the address filter configuration
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct AddrFilter(pub(super) pt_conf_addr_filter);
impl AddrFilter {
    pub const fn builder() -> AddrFilterBuilder {
        AddrFilterBuilder::new()
    }

    #[inline]
    pub fn addr0(&self) -> AddrRange {
        AddrRange::new(
            self.0.addr0_a,
            self.0.addr0_b,
            AddrConfig::try_from(unsafe { self.0.config.ctl.addr0_cfg() }).unwrap(),
        )
    }

    #[inline]
    pub fn addr1(&self) -> AddrRange {
        AddrRange::new(
            self.0.addr1_a,
            self.0.addr1_b,
            AddrConfig::try_from(unsafe { self.0.config.ctl.addr1_cfg() }).unwrap(),
        )
    }

    #[inline]
    pub fn addr2(&self) -> AddrRange {
        AddrRange::new(
            self.0.addr2_a,
            self.0.addr2_b,
            AddrConfig::try_from(unsafe { self.0.config.ctl.addr2_cfg() }).unwrap(),
        )
    }

    #[inline]
    pub fn addr3(&self) -> AddrRange {
        AddrRange::new(
            self.0.addr3_a,
            self.0.addr3_b,
            AddrConfig::try_from(unsafe { self.0.config.ctl.addr3_cfg() }).unwrap(),
        )
    }
}

#[derive(Debug)]
pub struct AddrFilterBuilder(pub(super) pt_conf_addr_filter);
impl Default for AddrFilterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AddrFilterBuilder {
    pub const fn new() -> Self {
        unsafe { mem::zeroed() }
    }

    #[inline]
    pub fn addr0(&mut self, range: AddrRange) -> &mut Self {
        self.0.addr0_a = range.a;
        self.0.addr0_b = range.b;
        unsafe { self.0.config.ctl.set_addr0_cfg(range.cfg as u32) };

        self
    }

    #[inline]
    pub fn addr1(&mut self, range: AddrRange) -> &mut Self {
        self.0.addr1_a = range.a;
        self.0.addr1_b = range.b;
        unsafe { self.0.config.ctl.set_addr1_cfg(range.cfg as u32) };

        self
    }

    #[inline]
    pub fn addr2(&mut self, range: AddrRange) -> &mut Self {
        self.0.addr2_a = range.a;
        self.0.addr2_b = range.b;
        unsafe { self.0.config.ctl.set_addr2_cfg(range.cfg as u32) };

        self
    }

    #[inline]
    pub fn addr3(&mut self, range: AddrRange) -> &mut Self {
        self.0.addr3_a = range.a;
        self.0.addr3_b = range.b;
        unsafe { self.0.config.ctl.set_addr3_cfg(range.cfg as u32) };

        self
    }

    pub fn build(&self) -> AddrFilter {
        AddrFilter(self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_addrfilter() {
        let filter = AddrFilter::builder()
            .addr0(AddrRange::new(1, 2, AddrConfig::DISABLED))
            .addr1(AddrRange::new(3, 4, AddrConfig::FILTER))
            .addr2(AddrRange::new(5, 6, AddrConfig::STOP))
            .addr3(AddrRange::new(7, 8, AddrConfig::DISABLED))
            .build();

        assert_eq!(filter.addr0().a(), 1);
        assert_eq!(filter.addr0().b(), 2);
        assert_eq!(filter.addr0().cfg(), AddrConfig::DISABLED);

        assert_eq!(filter.addr1().a(), 3);
        assert_eq!(filter.addr1().b(), 4);
        assert_eq!(filter.addr1().cfg(), AddrConfig::FILTER);

        assert_eq!(filter.addr2().a(), 5);
        assert_eq!(filter.addr2().b(), 6);
        assert_eq!(filter.addr2().cfg(), AddrConfig::STOP);

        assert_eq!(filter.addr3().a(), 7);
        assert_eq!(filter.addr3().b(), 8);
        assert_eq!(filter.addr3().cfg(), AddrConfig::DISABLED);
    }
}
