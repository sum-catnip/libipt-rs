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


// TODO: should Config really own pt_config? we'll need to copy on every callback..
// storing pt_config as reference has too many problems
// TODO: so uhm, i have no idea if the callback is stored
// in which case ill need to leak it?
// god testing this will be fucking awful

unsafe extern "C" fn decode_callback(ukn: *mut pt_packet_unknown,
                                     cfg: *const pt_config,
                                     pos: *const u8,
                                     ctx: *mut c_void) -> c_int {
    let c: &mut &mut dyn FnMut(&mut pt_packet_unknown, Config<()>, u8) -> i32
        = mem::transmute(ctx);
    c(&mut *ukn, (&*cfg).into(), *pos)
}

pub struct Config<'a, T> (pt_config, PhantomData<&'a T>);
impl<'a, T> Config<'a, T> {
    /// initializes a new Config instance for a trace without timing packets
    ///
    /// * `buffer` - the data captured by intelpt
    /// * `cpu`    - the cpu used for capturing. It's highly recommended to supply this info
    /// * `flags`  - a collection of decoder-specific flags
    /// * `filter` - the address filter configuration
    pub fn new_notiming<U, F>(buf:    &'a mut [u8],
                              cpu:    Option<CPU>,
                              flags:  Option<U>,
                              filter: Option<AddrFilter>,
                              decode: Option<F>) -> Self
                              where
                              U : Into<pt_conf_flags>,
                              F : FnMut(&mut pt_packet_unknown, Config<()>, u8) -> i32 {
        Config::new(buf, cpu, None, flags, filter, decode)
    }

    /// initializes a new Config instance for a trace with MTC timing packets enabled
    ///
    /// * `buf`    - the data captured by intelpt
    /// * `cpu`    - the cpu used for capturing. It's highly recommended to supply this info
    /// * `freq`   - frequency values used for timing packets
    /// * `flags`  - a collection of decoder-specific flags
    /// * `filter` - the address filter configuration
    pub fn new<U, F>(buf:    &'a mut [u8],
                     cpu:    Option<CPU>,
                     freq:   Option<Frequency>,
                     flags:  Option<U>,
                     filter: Option<AddrFilter>,
                     decode: Option<F>) -> Self
                     where
                     U : Into<pt_conf_flags>,
                     F : FnMut(&mut pt_packet_unknown, Config<()>, u8) -> i32 {
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
        if let Some(mut d) = decode {
            cfg.decode.callback = Some(decode_callback);
            cfg.decode.context  = &mut &mut d as *mut _ as *mut c_void;
        }

        Config(cfg, PhantomData)
    }
}

impl<'a, T> From<&pt_config> for Config<'_, T> {
    fn from(cfg: &pt_config) -> Self { Config(*cfg, PhantomData) }
}