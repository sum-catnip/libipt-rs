use super::cpu::Cpu;
use super::freqency::Frequency;
use super::filter::AddrFilter;
use crate::packet::unknown::Unknown;

use std::mem;
use std::any::Any;
use std::marker::PhantomData;
use std::ffi::c_void;
use std::os::raw::c_int;

use libipt_sys::{
    pt_config,
    pt_conf_flags,
    pt_packet_unknown
};

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::*;
    use crate::packet::unknown::Unknown;
    use std::any::TypeId;

    #[test]
    fn test_config_empty() {
        let c = Config::new(&mut [0; 0]);
        assert_eq!(c.0.begin, c.0.end);
        assert_eq!(c.0.size, mem::size_of::<pt_config>());
    }

    #[test]
    fn test_config_buf() {
        let mut data = [0; 16];
        let len = data.len();
        let c = Config::new(&mut data);
        assert_eq!(c.0.end as usize - c.0.begin as usize, len);
    }

    #[test]
    fn test_config_all() {
        let mut data = [18; 3];
        let mut c = Config::new(&mut data);
        c.set_cpu(Cpu::intel(1, 2, 3));
        c.set_freq(Frequency::new(1, 2, 3, 4));
        c.set_flags(BlockFlags::END_ON_CALL | BlockFlags::END_ON_JUMP);
        let mut f = AddrFilter::new();
        f.set_addr0(AddrRange::new(1, 2, AddrConfig::STOP));
        f.set_addr1(AddrRange::new(3, 4, AddrConfig::FILTER));
        f.set_addr2(AddrRange::new(5, 6, AddrConfig::DISABLED));
        f.set_addr3(AddrRange::new(7, 8, AddrConfig::STOP));
        c.set_filter(f);
        c.set_callback(|c, p| {
            match p[0] {
                17 => (Some(Box::new((c.0.cpu.model + 1) as u8)), 1),
                18 => (Some(Box::new("yeet".to_string())), 1),
                _ => unreachable!()
            }
        });

        assert_eq!(c.0.cpu.family, 1);
        assert_eq!(c.0.cpu.model, 2);
        assert_eq!(c.0.cpu.stepping, 3);

        assert_eq!(c.0.mtc_freq, 1);
        assert_eq!(c.0.nom_freq, 2);
        assert_eq!(c.0.cpuid_0x15_ebx, 3);
        assert_eq!(c.0.cpuid_0x15_eax, 4);

        unsafe {
            assert_eq!(c.0.flags.variant.block.end_on_call(), 1);
            assert_eq!(c.0.flags.variant.block.end_on_jump(), 1);
            assert_eq!(c.0.flags.variant.block.enable_tick_events(), 0);
            assert_eq!(c.0.flags.variant.block.keep_tcal_on_ovf(), 0);
        }

        assert_eq!(c.0.addr_filter.addr0_a, 1);
        assert_eq!(c.0.addr_filter.addr0_b, 2);
        assert_eq!(unsafe { c.0.addr_filter.config.ctl.addr0_cfg() },
                   AddrConfig::STOP as u32);

        assert_eq!(c.0.addr_filter.addr1_a, 3);
        assert_eq!(c.0.addr_filter.addr1_b, 4);
        assert_eq!(unsafe { c.0.addr_filter.config.ctl.addr1_cfg() },
                   AddrConfig::FILTER as u32);

        assert_eq!(c.0.addr_filter.addr2_a, 5);
        assert_eq!(c.0.addr_filter.addr2_b, 6);
        assert_eq!(unsafe { c.0.addr_filter.config.ctl.addr2_cfg() },
                   AddrConfig::DISABLED as u32);

        assert_eq!(c.0.addr_filter.addr3_a, 7);
        assert_eq!(c.0.addr_filter.addr3_b, 8);
        assert_eq!(unsafe { c.0.addr_filter.config.ctl.addr3_cfg() },
                   AddrConfig::STOP as u32);

        unsafe {
            let mut ukn: pt_packet_unknown = std::mem::zeroed();
            assert_eq!(
                c.0.decode.callback.unwrap()(&mut ukn,
                                             &c.0, c.0.begin,
                                             c.0.decode.context),
                1);
            let pkt: Unknown = ukn.into();
            println!("{:?}", pkt.data().unwrap().type_id());
            assert!(pkt.data().unwrap().is::<Box<String>>());
        }
    }

    fn check_callback(cfg: &mut Config, expect: i32) -> bool {
        unsafe {
            let mut ukn: pt_packet_unknown = std::mem::zeroed();
            return
                cfg.0.decode.callback.unwrap()(&mut ukn,
                                             &cfg.0, &11,
                                             cfg.0.decode.context)
                == expect
        }
    }

    fn test_boxany() -> Box<Box<dyn Any>> {
        Box::new(Box::new("yeet".to_string()))
    }

    #[test]
    fn test() {
        // t: 9832638357655698176
        println!("String {:?}", TypeId::of::<String>());
        let orig = test_boxany();
        // passes
        assert!((&*orig).is::<String>());
        // t: 9832638357655698176
        println!("orig1 {:?}", (&*orig).type_id());
        // leak box
        let raw: *mut c_void = Box::into_raw(orig) as *mut _;
        // catch box
        let raw = raw as *mut Box<dyn Any>;
        // t: 11266574366495284750
        unsafe { println!("raw2 {:?}", (*raw).type_id()) };
        let orig = unsafe { Box::from_raw(raw) };
        // t: 11266574366495284750
        println!("orig2 {:?}", (&*orig).type_id());
        println!("{:?}", orig);
        // panics
        assert!((&*orig).is::<String>());
    }
/*
    #[test]
    fn test_config_callback_safety() {
        let mut cfg = Config::new(&mut [0;0]);
        cfg.set_callback(|_, _, p| (p + 6) as i32 );

        for _ in 0..10 { assert!(check_callback(&mut cfg, 17)) }
        cfg.set_callback(|_, _, p| (p + 3) as i32);
        for _ in 0..10 { assert!(check_callback(&mut cfg, 14)) }
    }*/
}


// TODO
// potentially make a type for reading packet bytes from the current position
// i think the user defined thingy should be provided within the callback
// kina sounds like it:
// The *pt_packet_unknown* object can be used to provide user-defined
// information back to the user when using the packet decoder to iterate over
// Intel PT packets.  Other decoders ignore this information but will skip
// the packet if a non-zero size is returned by the callback function.

// i should think about what happens to the pt_config ptr in the callback
// does it get copied? moved? pretty sure it copied
// but do i want a copy on each callback?

// also i need to think about the priv_ type of the unknown packet
// i have no fucking idea how to propagate the type

unsafe extern "C" fn decode_callback<'a, F>(ukn: *mut pt_packet_unknown,
                                            cfg: *const pt_config,
                                            pos: *const u8,
                                            ctx: *mut c_void) -> c_int
    where F: FnMut(&Config, &[u8]) -> (Unknown<'a>, u32) {

    let sz = (*cfg).end as usize - pos as usize;
    let pos = std::slice::from_raw_parts(pos, sz);

    let c = ctx as *mut F;
    let c = &mut *c;

    let (res, bytes) = c(&(&*cfg).into(), pos);
    // TODO
    // REMEMBER TO CATCH THE BOX FROM THE DECODER
    (*ukn).priv_ = match res {
        Some(r) => Box::into_raw(r) as *mut _,
        None => std::ptr::null_mut()
    };

    bytes as i32
}

/// A libipt configuration
pub struct Config<'a> (pub(crate) pt_config, PhantomData<&'a ()>);
impl<'a> Config<'a> {
    /// Initializes a Config instance with only a buffer.
    ///
    /// Chain this functions with the setter methods to provide the arguments you need
    pub fn new(buf: &'a mut [u8]) -> Self {
        let mut cfg: pt_config = unsafe { mem::zeroed() };
        cfg.size  = mem::size_of::<pt_config>();
        cfg.begin = buf.as_mut_ptr();
        cfg.end   = unsafe { buf.as_mut_ptr().offset(buf.len() as isize) };
        Config(cfg, PhantomData)
    }

    /// The cpu used for capturing the data.
    /// It's highly recommended to provide this information.
    /// Processor specific workarounds will be identified this way.
    #[inline]
    pub fn set_cpu(&mut self, cpu: Cpu) {
        self.0.cpu = cpu.0;
        self.0.errata = cpu.determine_errata();
    }

    /// Frequency values used for timing packets (mtc)
    #[inline]
    pub fn set_freq(&mut self, freq: Frequency) {
        self.0.mtc_freq = freq.mtc;
        self.0.nom_freq = freq.nom;
        self.0.cpuid_0x15_eax = freq.tsc;
        self.0.cpuid_0x15_ebx = freq.ctc;
    }

    /// Decoder specific flags
    #[inline]
    pub fn set_flags(&mut self, flags: impl Into<pt_conf_flags>) {
        self.0.flags = flags.into();
    }

    /// Address filter configuration
    #[inline]
    pub fn set_filter(&mut self, filter: AddrFilter) {
        self.0.addr_filter = filter.0;
    }

    /// A callback for decoding unknown packets
    #[inline]
    pub fn set_callback<'b, F>(&mut self, mut cb: F)
        where F: FnMut(&Config, &[u8]) -> (Option<Box<dyn Any>>, u32),
              F: 'a {

        self.0.decode.callback = Some(decode_callback::<F>);
        self.0.decode.context  = &mut cb as *mut _ as *mut c_void;
    }
}

impl<'a> From<&pt_config> for Config<'_> {
    fn from(cfg: &pt_config) -> Self {
        Config(*cfg, PhantomData)
    }
}