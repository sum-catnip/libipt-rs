use crate::error::{PtError, PtErrorCode};
use libipt_sys::pt_conf_addr_filter;
use num_enum::TryFromPrimitive;
use std::mem::{transmute, zeroed};

/// This corresponds to the IA32_RTIT_ADDRn_CFG MSRs
#[derive(Clone, Copy, TryFromPrimitive, PartialEq, Debug, Default)]
#[repr(u32)]
pub enum AddrFilterType {
    #[default]
    DISABLED = 0,
    FILTER = 1,
    STOP = 2,
}

impl AddrFilterType {
    #[must_use]
    const fn to_addr_cfg_raw(self, n: u32) -> u64 {
        (self as u64) << (4 * n)
    }
}

/// an address range inside the address filter
#[derive(Debug, Clone, Copy)]
pub struct AddrFilter {
    /// This corresponds to the IA32_RTIT_ADDRn_A MSRs
    pub from: u64,
    /// This corresponds to the IA32_RTIT_ADDRn_B MSRs
    pub to: u64,
    /// this corresponds to the respective fields in IA32_RTIT_CTL MSR
    pub filter_type: AddrFilterType,
}

impl AddrFilter {
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
pub struct AddrFilters(pub(crate) pt_conf_addr_filter);

impl AddrFilters {
    pub const fn new(filters: &[AddrFilter]) -> Result<Self, PtError> {
        if filters.len() > 4 {
            return Err(PtError::new(
                PtErrorCode::BadConfig,
                "The maximum number of address filters is 4",
            ));
        }
        let mut inner = unsafe { zeroed::<pt_conf_addr_filter>() };
        // TODO improve this when something better will become const, for example get and the
        // bindgen methods on config bitfields
        if !filters.is_empty() {
            (inner.addr0_a, inner.addr0_b) = (filters[0].from, filters[0].to);
            unsafe { inner.config.addr_cfg |= filters[0].filter_type.to_addr_cfg_raw(0) };
        }
        if filters.len() >= 2 {
            (inner.addr1_a, inner.addr1_b) = (filters[1].from, filters[1].to);
            unsafe { inner.config.addr_cfg |= filters[1].filter_type.to_addr_cfg_raw(1) };
        }
        if filters.len() >= 3 {
            (inner.addr2_a, inner.addr2_b) = (filters[2].from, filters[2].to);
            unsafe { inner.config.addr_cfg |= filters[2].filter_type.to_addr_cfg_raw(2) };
        }
        if filters.len() == 4 {
            (inner.addr3_a, inner.addr3_b) = (filters[3].from, filters[3].to);
            unsafe { inner.config.addr_cfg |= filters[3].filter_type.to_addr_cfg_raw(3) };
        }

        Ok(Self(inner))
    }

    #[inline]
    const fn addrn_filter_type(&self, n: u32) -> AddrFilterType {
        unsafe { transmute((self.0.config.addr_cfg as u32 >> (4 * n)) & 0xf) }
    }

    pub const fn addr0(&self) -> AddrFilter {
        AddrFilter {
            from: self.0.addr0_a,
            to: self.0.addr0_b,
            filter_type: self.addrn_filter_type(0),
        }
    }

    pub const fn addr1(&self) -> AddrFilter {
        AddrFilter {
            from: self.0.addr1_a,
            to: self.0.addr1_b,
            filter_type: self.addrn_filter_type(1),
        }
    }

    pub const fn addr2(&self) -> AddrFilter {
        AddrFilter {
            from: self.0.addr2_a,
            to: self.0.addr2_b,
            filter_type: self.addrn_filter_type(2),
        }
    }

    pub const fn addr3(&self) -> AddrFilter {
        AddrFilter {
            from: self.0.addr3_a,
            to: self.0.addr3_b,
            filter_type: self.addrn_filter_type(3),
        }
    }

    pub const fn set_addr0(&mut self, range: AddrFilter) {
        self.0.addr0_a = range.from;
        self.0.addr0_b = range.to;
        unsafe { self.0.config.addr_cfg |= range.filter_type.to_addr_cfg_raw(0) };
    }

    pub const fn set_addr1(&mut self, range: AddrFilter) {
        self.0.addr1_a = range.from;
        self.0.addr1_b = range.to;
        unsafe { self.0.config.addr_cfg |= range.filter_type.to_addr_cfg_raw(1) };
    }

    pub const fn set_addr2(&mut self, range: AddrFilter) {
        self.0.addr2_a = range.from;
        self.0.addr2_b = range.to;
        unsafe { self.0.config.addr_cfg |= range.filter_type.to_addr_cfg_raw(2) };
    }

    #[inline]
    pub const fn set_addr3(&mut self, range: AddrFilter) {
        self.0.addr3_a = range.from;
        self.0.addr3_b = range.to;
        unsafe { self.0.config.addr_cfg |= range.filter_type.to_addr_cfg_raw(3) };
    }

    pub const fn iter(&self) -> Iter<'_> {
        Iter::new(self)
    }
}

impl Default for AddrFilters {
    fn default() -> Self {
        Self(unsafe { zeroed() })
    }
}

impl PartialEq for AddrFilters {
    fn eq(&self, other: &Self) -> bool {
        (
            self.0.addr0_a,
            self.0.addr0_b,
            self.0.addr1_a,
            self.0.addr1_b,
            self.0.addr2_a,
            self.0.addr2_b,
            self.0.addr3_a,
            self.0.addr3_b,
        ) == (
            other.0.addr0_a,
            other.0.addr0_b,
            other.0.addr1_a,
            other.0.addr1_b,
            other.0.addr2_a,
            other.0.addr2_b,
            other.0.addr3_a,
            other.0.addr3_b,
        ) && unsafe { self.0.config.addr_cfg == other.0.config.addr_cfg }
    }
}

pub struct Iter<'a> {
    filters: &'a AddrFilters,
    i: usize,
}

impl<'a> Iter<'a> {
    const fn new(filters: &'a AddrFilters) -> Self {
        Self { filters, i: 0 }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = AddrFilter;

    fn next(&mut self) -> Option<Self::Item> {
        let filter = match self.i {
            0 => self.filters.addr0(),
            1 => self.filters.addr1(),
            2 => self.filters.addr2(),
            3 => self.filters.addr3(),
            _ => return None,
        };
        self.i += 1;
        Some(filter)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_addrfilter() {
        let filter = AddrFilters::new(&[
            AddrFilter::new(1, 2, AddrFilterType::DISABLED),
            AddrFilter::new(3, 4, AddrFilterType::FILTER),
            AddrFilter::new(5, 6, AddrFilterType::STOP),
            AddrFilter::new(7, 8, AddrFilterType::DISABLED),
        ])
        .unwrap();

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
