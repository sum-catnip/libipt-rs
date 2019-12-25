use super::flags::DecoderFlags;
use super::cpu::CPU;
use super::freqency::Frequency;
use super::filter::AddrFilter;

use std::ptr;
use libipt_sys::{
    pt_config,
    pt_config__bindgen_ty_1,
};

/* TODO: expose rustic callback with ok performance..
pub trait DecodeUnknown {
    pub fn decode(&mut self, pck: pt_packet_unknown, )
}*/


pub struct Config (pt_config);
impl Config {
    /// initializes a new Config instance for a trace without timing packets
    /// 
    /// * `buffer` - the data captured by intelpt
    /// * `cpu` - the cpu used for capturing. It's highly recommended to supply this info
    /// * `flags` - a collection of decoder-specific flags
    /// * `filter` - the address filter configuration
    pub fn new_notiming(buffer: &mut [u8],
                        cpu:    Option<CPU>,
                        flags:  Option<DecoderFlags>,
                        filter: Option<AddrFilter>) -> Self {
        Config::new(buffer, cpu, None, flags, filter)
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
    pub fn new(buffer: &mut [u8],
               cpu:    Option<CPU>,
               freq:   Option<Frequency>,
               flags:  Option<DecoderFlags>,
               filter: Option<AddrFilter>) -> Self {
        // TODO error handling if buffer has no elements
        // would i really want to return Result<Config>?
        // seems a bit weird to have a failing ctor
        // maybe im just an oop slut
        let buffer_ptr = buffer.as_mut_ptr();
        let freq = freq.unwrap_or_default();
        let cpu = cpu.unwrap_or_default();

        Config (
            pt_config {
                cpu: cpu.into(),
                flags: flags.unwrap_or_default().into(),
                errata: cpu.determine_errata(),
                size: buffer.len(),
                begin: buffer_ptr,
                end: unsafe { buffer_ptr.offset(buffer.len() as isize) },
                mtc_freq: freq.mtc(),
                nom_freq: freq.nom(),
                cpuid_0x15_eax: freq.tsc(),
                cpuid_0x15_ebx: freq.ctc(),
                addr_filter: filter.unwrap_or_default().into(),
                decode: pt_config__bindgen_ty_1 {
                    callback: None,
                    context: ptr::null_mut()
                }
            }
        )
    }
}