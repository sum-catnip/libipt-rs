use libipt_sys::pt_conf_addr_filter;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::mem;

/// This corresponds to the IA32_RTIT_ADDRn_CFG MSRs
#[derive(Clone, Copy, TryFromPrimitive, PartialEq, Debug)]
#[repr(u32)]
pub enum AddrFilterType {
    DISABLED = 0,
    FILTER = 1,
    STOP = 2,
}

/// an address range inside the address filter
#[derive(Debug, Clone, Copy)]
pub struct AddrFilterRange {
    /// This corresponds to the IA32_RTIT_ADDRn_A MSRs
    pub from: u64,
    /// This corresponds to the IA32_RTIT_ADDRn_B MSRs
    pub to: u64,
    /// this corresponds to the respective fields in IA32_RTIT_CTL MSR
    pub filter_type: AddrFilterType,
}

impl AddrFilterRange {
    #[inline]
    pub const fn new(from: u64, to: u64, filter_type: AddrFilterType) -> Self {
        Self {
            from,
            to,
            filter_type,
        }
    }
}

/// the address filter configuration
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct AddrFilters(pub(super) pt_conf_addr_filter);
impl AddrFilters {
    pub const fn builder() -> AddrFiltersBuilder {
        AddrFiltersBuilder::new()
    }

    #[inline]
    pub fn addr0(&self) -> AddrFilterRange {
        AddrFilterRange {
            from: self.0.addr0_a,
            to: self.0.addr0_b,
            filter_type: AddrFilterType::try_from(unsafe { self.0.config.ctl.addr0_cfg() })
                .unwrap(),
        }
    }

    #[inline]
    pub fn addr1(&self) -> AddrFilterRange {
        AddrFilterRange {
            from: self.0.addr1_a,
            to: self.0.addr1_b,
            filter_type: AddrFilterType::try_from(unsafe { self.0.config.ctl.addr1_cfg() })
                .unwrap(),
        }
    }

    #[inline]
    pub fn addr2(&self) -> AddrFilterRange {
        AddrFilterRange {
            from: self.0.addr2_a,
            to: self.0.addr2_b,
            filter_type: AddrFilterType::try_from(unsafe { self.0.config.ctl.addr2_cfg() })
                .unwrap(),
        }
    }

    #[inline]
    pub fn addr3(&self) -> AddrFilterRange {
        AddrFilterRange {
            from: self.0.addr3_a,
            to: self.0.addr3_b,
            filter_type: AddrFilterType::try_from(unsafe { self.0.config.ctl.addr3_cfg() })
                .unwrap(),
        }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct AddrFiltersBuilder(pub(super) pt_conf_addr_filter);
impl Default for AddrFiltersBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AddrFiltersBuilder {
    pub const fn new() -> Self {
        unsafe { mem::zeroed() }
    }

    #[inline]
    pub fn addr0(&mut self, range: AddrFilterRange) -> &mut Self {
        self.0.addr0_a = range.from;
        self.0.addr0_b = range.to;
        unsafe { self.0.config.ctl.set_addr0_cfg(range.filter_type as u32) };

        self
    }

    #[inline]
    pub fn addr1(&mut self, range: AddrFilterRange) -> &mut Self {
        self.0.addr1_a = range.from;
        self.0.addr1_b = range.to;
        unsafe { self.0.config.ctl.set_addr1_cfg(range.filter_type as u32) };

        self
    }

    #[inline]
    pub fn addr2(&mut self, range: AddrFilterRange) -> &mut Self {
        self.0.addr2_a = range.from;
        self.0.addr2_b = range.to;
        unsafe { self.0.config.ctl.set_addr2_cfg(range.filter_type as u32) };

        self
    }

    #[inline]
    pub fn addr3(&mut self, range: AddrFilterRange) -> &mut Self {
        self.0.addr3_a = range.from;
        self.0.addr3_b = range.to;
        unsafe { self.0.config.ctl.set_addr3_cfg(range.filter_type as u32) };

        self
    }

    pub fn build(&self) -> AddrFilters {
        AddrFilters(self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_addrfilter() {
        let filter = AddrFilters::builder()
            .addr0(AddrFilterRange::new(1, 2, AddrFilterType::DISABLED))
            .addr1(AddrFilterRange::new(3, 4, AddrFilterType::FILTER))
            .addr2(AddrFilterRange::new(5, 6, AddrFilterType::STOP))
            .addr3(AddrFilterRange::new(7, 8, AddrFilterType::DISABLED))
            .build();

        assert_eq!(filter.addr0().from, 1);
        assert_eq!(filter.addr0().to, 2);
        assert_eq!(filter.addr0().filter_type, AddrFilterType::DISABLED);

        assert_eq!(filter.addr1().from, 3);
        assert_eq!(filter.addr1().to, 4);
        assert_eq!(filter.addr1().filter_type, AddrFilterType::FILTER);

        assert_eq!(filter.addr2().from, 5);
        assert_eq!(filter.addr2().to, 6);
        assert_eq!(filter.addr2().filter_type, AddrFilterType::STOP);

        assert_eq!(filter.addr3().from, 7);
        assert_eq!(filter.addr3().to, 8);
        assert_eq!(filter.addr3().filter_type, AddrFilterType::DISABLED);
    }
}
