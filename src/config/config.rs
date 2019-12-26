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

/* TODO: expose rustic callback with ok performance..
pub trait DecodeUnknown {
    pub fn decode(&mut self, pck: pt_packet_unknown, )
}*/

// TODO: should Config really own pt_config? we'll need to copy on every callback..

unsafe extern "C" fn kek(ukn: *mut pt_packet_unknown,
                         cfg: *const pt_config,
                         pos: *const u8,
                         ctx: *mut c_void) -> c_int {
    
}

pub struct Config<'a, T> (pt_config, PhantomData<&'a T>);
impl<'a, T> Config<'a, T> {
    /// initializes a new Config instance for a trace without timing packets
    ///
    /// * `buffer` - the data captured by intelpt
    /// * `cpu`    - the cpu used for capturing. It's highly recommended to supply this info
    /// * `flags`  - a collection of decoder-specific flags
    /// * `filter` - the address filter configuration
    pub fn new_notiming<U>(buf:    &'a mut [u8],
                           cpu:    Option<CPU>,
                           flags:  Option<U>,
                           filter: Option<AddrFilter>) -> Self
                           where U : Into<pt_conf_flags> {
        Config::new(buf, cpu, None, flags, filter)
    }

    /// initializes a new Config instance for a trace with MTC timing packets enabled
    ///
    /// * `buf`    - the data captured by intelpt
    /// * `cpu`    - the cpu used for capturing. It's highly recommended to supply this info
    /// * `freq`   - frequency values used for timing packets
    /// * `flags`  - a collection of decoder-specific flags
    /// * `filter` - the address filter configuration
    pub fn new<U>(buf:    &'a mut [u8],
                  cpu:    Option<CPU>,
                  freq:   Option<Frequency>,
                  flags:  Option<U>,
                  filter: Option<AddrFilter>) -> Config<'_, T>
                  where U : Into<pt_conf_flags> {
        // TODO error handling if buffer has no elements
        // would i really want to return Result<Config>?
        // seems a bit weird to have a failing ctor
        // maybe im just an oop slut

        // thats how its done in the libipt docs so itll be fine
        let mut cfg: pt_config = unsafe { mem::zeroed() };
        cfg.begin = buf.as_mut_ptr();
        cfg.end   = unsafe { buf.as_mut_ptr().offset(buf.len() as isize) };

        if let Some(c) = cpu {
             cfg.cpu = c.0;
             cfg.errata = c.determine_errata();
        }

        if let Some(f) = freq {
            cfg.mtc_freq = f.mtc;
            cfg.nom_freq = f.nom;
            cfg.cpuid_0x15_eax = f.tsc;
            cfg.cpuid_0x15_ebx = f.ctc;
        }

        if let Some(f) = flags { cfg.flags = f.into() }
        if let Some(f) = filter { cfg.addr_filter = f.0 }

        Config(cfg, PhantomData)
    }
}

impl<'a, T> From<&pt_config> for Config<'_, T> {
    fn from(cfg: &pt_config) -> Self { Config(*cfg, PhantomData) }
}