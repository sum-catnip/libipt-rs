use libipt_sys::{
    pt_conf_addr_filter,
    pt_conf_addr_filter__bindgen_ty_1
};

/// an address range inside the address filter
#[derive(Clone, Copy, Default)]
pub struct AddrRange {
    /// This corresponds to the IA32_RTIT_ADDRn_A MSRs
    a: u64,
    /// This corresponds to the IA32_RTIT_ADDRn_B MSRs
    b: u64,
    /// this corresponds to the respective fields in IA32_RTIT_CTL MSR
    cfg: u64
}

/// the address filter configuration
#[derive(Clone, Copy, Default)]
pub struct AddrFilter (
    AddrRange,
    AddrRange,
    AddrRange,
    AddrRange
);

impl From<AddrFilter> for pt_conf_addr_filter {
    fn from(filter: AddrFilter) -> Self {
        pt_conf_addr_filter {
            reserved: Default::default(),
            addr0_a: filter.0.a,
            addr0_b: filter.0.b,
            addr1_a: filter.1.a,
            addr1_b: filter.1.b,
            addr2_a: filter.2.a,
            addr2_b: filter.2.b,
            addr3_a: filter.3.a,
            addr3_b: filter.3.b,
            config: pt_conf_addr_filter__bindgen_ty_1 {
                addr_cfg: (filter.3.cfg << 12) |
                          (filter.2.cfg << 8)  |
                          (filter.1.cfg << 4)  |
                          (filter.0.cfg)
            }
        }
    }
}