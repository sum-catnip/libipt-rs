use std::ptr;
use libipt_sys::{
    pt_config,
    pt_config__bindgen_ty_1,
    pt_packet_unknown,
    pt_cpu,
    pt_cpu_vendor,
    pt_cpu_vendor_pcv_intel,
    pt_cpu_vendor_pcv_unknown,
    pt_errata,
    pt_cpu_errata,
    pt_conf_flags,
    pt_conf_flags__bindgen_ty_1,
    pt_conf_flags__bindgen_ty_1__bindgen_ty_1,
    pt_conf_flags__bindgen_ty_1__bindgen_ty_2,
    pt_conf_flags__bindgen_ty_1__bindgen_ty_3,
    pt_conf_addr_filter__bindgen_ty_1,
    pt_conf_addr_filter,
    __BindgenBitfieldUnit
};
use bitflags::bitflags;

mod test {
    use super::*;
}

/* TODO: expose rustic callback with ok performance..
pub trait DecodeUnknown {
    pub fn decode(&mut self, pck: pt_packet_unknown, )
}*/

bitflags! {
    /// flags for the block decoder
    pub struct BlockFlags: u8 {
        /// End a block after a call instruction
        const END_ON_CALL        = 0b00000001;
        /// Enable tick events for timing updates
        const ENABLE_TICK_EVENTS = 0b00000010;
        /// End a block after a jump instruction
        const END_ON_JUMP        = 0b00000100;
        /// Preserve timing calibration on overflow
        const KEEP_TCAL_ON_OVF   = 0b00001000;
    }
}

bitflags! {
    /// flags for the instruction flow decoder
    pub struct InsnFlags: u8 {
        /// Enable tick events for timing updates
        const ENABLE_TICK_EVENTS = 0b00000001;
        /// Preserve timing calibration on overflow
        const KEEP_TCAL_ON_OVF   = 0b00000010;
    }
}

bitflags! {
    /// flags for the query decoder
    pub struct QueryFlags: u8 {
        /// Preserve timing calibration on overflow
        const KEEP_TCAL_ON_OVF = 0b00000001;
    }
}

/// a collection of decoder-specific configuration flags
pub enum DecoderFlags {
    /// flags for the block decoder
    Block(BlockFlags),
    /// flags for the instruction flow decoder
    Insn(InsnFlags),
    /// flags for the query decoder
    Query(QueryFlags)
}

impl From<DecoderFlags> for pt_conf_flags {
    fn from(flags: DecoderFlags) -> Self {
        // yeah i know.. this looks absolutely disgusting
        // i just wanted a decent rustic abstraction for the union fields
        match flags {
            DecoderFlags::Block(f) => pt_conf_flags {
                variant: pt_conf_flags__bindgen_ty_1 {
                    block: pt_conf_flags__bindgen_ty_1__bindgen_ty_1 {
                        _bitfield_1: __BindgenBitfieldUnit::new([f.bits()]),
                        __bindgen_padding_0: Default::default() } } },

            DecoderFlags::Insn(f) => pt_conf_flags {
                variant: pt_conf_flags__bindgen_ty_1 {
                    insn: pt_conf_flags__bindgen_ty_1__bindgen_ty_2 {
                        _bitfield_1: __BindgenBitfieldUnit::new([f.bits()]),
                        __bindgen_padding_0: Default::default() } } },

            DecoderFlags::Query(f) => pt_conf_flags {
                variant: pt_conf_flags__bindgen_ty_1 {
                    query: pt_conf_flags__bindgen_ty_1__bindgen_ty_3 {
                        _bitfield_1: __BindgenBitfieldUnit::new([f.bits()]),
                        __bindgen_padding_0: Default::default() } } },
       }
    }
}

bitflags! {
    /// i suppose this is relevant when/if amd finally gets intelpt support
    pub struct CPUVendor: i32 {
        const INTEL = pt_cpu_vendor_pcv_intel;
        const UNKNOWN = pt_cpu_vendor_pcv_unknown;
    }
}

/// a cpu identifier
pub struct CPU (pt_cpu);
impl CPU {
    pub fn new(vendor: CPUVendor, family: u16, model: u8, stepping: u8) -> Self {
        CPU(pt_cpu{
            vendor: vendor.bits(),
            family, model, stepping
        })
    }
}

/// cpuid_0x15_eax, cpuid_0x15_ebx : The values of eax and ebx on a cpuid call for leaf 0x15.
///
// The value *ebx/eax* gives the ratio of the Core Crystal Clock (CTC) to
// Timestamp Counter (TSC) frequency.
///
// This field is ignored by the packet encoder and packet decoder. It is
// required for other decoders if Mini Time Counter (MTC) packets are enabled
// in the collected trace.
#[derive(Clone, Copy, Default)]
pub struct CTCTSCRatio {
    /// the values of eax and ebx on a cpuid call for leaf 0x15
    cpuid_0x15_eax: u32,
    /// the values of eax and ebx on a cpuid call for leaf 0x15
    cpuid_0x15_ebx: u32
}

impl CTCTSCRatio {
    pub fn new(cpuid_0x15_eax: u32, cpuid_0x15_ebx: u32) -> Self {
        CTCTSCRatio { cpuid_0x15_eax, cpuid_0x15_ebx }
    }
}

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

pub struct Config (pt_config);
impl Config {
    /// initializes a new Config instance for a trace without timing packets
    /// 
    /// * `buffer` - the data captured by intelpt
    /// * `cpu` - the cpu used for capturing. It's highly recommended to supply this info
    /// * `flags` - a collection of decoder-specific flags
    /// * `filter` - the address filter configuration
    pub fn new_notiming(
        buffer: &mut [u8], cpu: Option<CPU>,
        flags: Option<DecoderFlags>,
        filter: Option<AddrFilter>
    ) -> Self {
        Config::new_timing(
            buffer, cpu,
            Default::default(),
            Default::default(),
            Default::default(),
            flags, filter
        )
    }

    /// initializes a new Config instance for a trace with MTC timing packets enabled
    ///
    /// * `buffer` - the data captured by intelpt
    /// * `cpu` - the cpu used for capturing. It's highly recommended to supply this info
    /// * `ratio` - the CTC frequency
    /// * `mtc_freq` - the MTC frequency as defined in IA32_RTIT_CTL.MTCFreq
    /// * `nom_freq` - the nominal frequency as defined in MSR_PLATFORM_INFO[15:8]
    /// if zero, timing calibration will only be able to use MTC and CYC packets
    /// if not zero, timing calibration will also be able to use CBR packets
    /// * `flags` - a collection of decoder-specific flags
    /// * `filter` - the address filter configuration
    pub fn new_timing(
        buffer: &mut [u8],
        cpu: Option<CPU>,
        ratio: CTCTSCRatio,
        mtc_freq: u8, nom_freq: u8,
        flags: Option<DecoderFlags>,
        filter: Option<AddrFilter>
    ) -> Self {
        // TODO error handling if buffer has no elements
        // would i really want to return Result<Config>?
        // seems a bit weird to have a failing ctor
        // maybe im just an oop slut
        let mut errata = pt_errata{
            _bitfield_1: Default::default(),
            reserved: Default::default()
        };

        // determine errata if cpu is supplied
        // conveniently also unwraps the cpu option
        let cpu = match cpu {
            None => pt_cpu { vendor: 0, family: 0, model: 0, stepping: 0 },
            Some(cpu) => {
                unsafe{pt_cpu_errata(&mut errata, &cpu.0);}
                cpu.0
            }
        };

        // if flags is none, we create a default pt_conf_flags
        let flags = match flags {
            Some(f) => f.into(),
            None => pt_conf_flags {
                variant: pt_conf_flags__bindgen_ty_1 {
                    block: pt_conf_flags__bindgen_ty_1__bindgen_ty_1 {
                        _bitfield_1: Default::default(),
                        __bindgen_padding_0: Default::default() }}},
        };

        let buffer_ptr = buffer.as_mut_ptr();

        Config (
            pt_config {
                cpu, flags, errata, mtc_freq, nom_freq,
                size: buffer.len(),
                begin: buffer_ptr,
                end: unsafe { buffer_ptr.offset(buffer.len() as isize) },
                cpuid_0x15_eax: ratio.cpuid_0x15_eax,
                cpuid_0x15_ebx: ratio.cpuid_0x15_ebx,
                addr_filter: filter.unwrap_or_default().into(),
                decode: pt_config__bindgen_ty_1 {
                    callback: None,
                    context: ptr::null_mut()
                }
            }
        )
    }
}