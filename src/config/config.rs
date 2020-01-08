use super::cpu::CPU;
use super::freqency::Frequency;
use super::filter::AddrFilter;

use std::mem;
use std::marker::PhantomData;
use std::ffi::c_void;
use std::os::raw::c_int;

use libipt_sys::{
    pt_config,
    pt_conf_flags,
    pt_packet_unknown
};


// TODO: should Config really own pt_config? how does moving it every callback even work
// i think the callback is cheating rust since the pt_config doesnt have a lifetime
// not sure if that matters since its readonly anyways
// TODO: so uhm, i have no idea if the callback is stored
// in which case ill need to leak it?
// god testing this will be fucking awful

// MOAH TODO: see how the decode callback behaves
// potentially make a type for reading packet bytes from the current position
// i think the user defined thingy should be provided within the callback
// kina sounds like it:
// The *pt_packet_unknown* object can be used to provide user-defined
// information back to the user when using the packet decoder to iterate over
// Intel PT packets.  Other decoders ignore this information but will skip
// the packet if a non-zero size is returned by the callback function.


unsafe extern "C" fn decode_callback(ukn: *mut pt_packet_unknown,
                                     cfg: *const pt_config,
                                     pos: *const u8,
                                     ctx: *mut c_void) -> c_int {
    let c: &mut &mut dyn FnMut(&mut pt_packet_unknown, Config, u8) -> i32
        = mem::transmute(ctx);
    c(&mut *ukn, (&*cfg).into(), *pos)
}

/// A builder type for the libipt configuration
#[derive(Clone, Copy)]
pub struct Config<'a> (pub(crate) pt_config, PhantomData<&'a ()>);
impl<'a> Config<'a> {
    /// Initializes a Config instance with only a buffer.
    ///
    /// Chain this functions with the setter methods to provide the arguments you need
    pub fn new(buf: &'a mut [u8]) -> Self {
        let mut cfg: pt_config = unsafe { mem::zeroed() };
        cfg.begin = buf.as_mut_ptr();
        cfg.end   = unsafe { buf.as_mut_ptr().offset(buf.len() as isize) };
        Config(cfg, PhantomData)
    }

    /// The cpu used for capturing the data.
    /// It's highly recommended to provide this information.
    /// Processor specific workarounds will be identified this way.
    #[inline]
    pub fn cpu(&mut self, cpu: CPU) -> &mut Self {
        self.0.cpu = cpu.0;
        self.0.errata = cpu.determine_errata();

        self
    }

    /// Frequency values used for timing packets (mtc)
    #[inline]
    pub fn freq(&mut self, freq: Frequency) -> &mut Self {
        self.0.mtc_freq = freq.mtc;
        self.0.nom_freq = freq.nom;
        self.0.cpuid_0x15_eax = freq.tsc;
        self.0.cpuid_0x15_ebx = freq.ctc;

        self
    }

    /// Decoder specific flags
    #[inline]
    pub fn flags(&mut self, flags: impl Into<pt_conf_flags>) -> &mut Self {
        self.0.flags = flags.into();

        self
    }

    /// Address filter configuration
    #[inline]
    pub fn filter(&mut self, filter: AddrFilter) -> &mut Self {
        self.0.addr_filter = filter.0;

        self
    }

    /// A callback for decoding unknown packets
    #[inline]
    pub fn callback(&mut self,
                    mut cb: impl FnMut(&mut pt_packet_unknown, Config, u8) -> i32)
                    -> &mut Self {
        self.0.decode.callback = Some(decode_callback);
        self.0.decode.context  = &mut &mut cb as *mut _ as *mut c_void;

        self
    }
}

impl<'a> From<&pt_config> for Config<'_> {
    fn from(cfg: &pt_config) -> Self { Config(*cfg, PhantomData) }
}