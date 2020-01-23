use super::cpu::Cpu;
use super::freqency::Frequency;
use super::filter::AddrFilter;
use crate::packet::unknown::Unknown;

use std::mem;
use std::borrow::Cow;
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

    #[test]
    fn test_config_empty() {
        let c = Config::<()>::new(&mut [0; 0]);
        assert_eq!(c.0.begin, c.0.end);
        assert_eq!(c.0.size, mem::size_of::<pt_config>());
    }

    #[test]
    fn test_config_buf() {
        let mut data = [0; 16];
        let len = data.len();
        let c = Config::<()>::new(&mut data);
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
            (Unknown::new(c.0.cpu.model + p[0]), 1)
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
                                             c.0.as_ref(), c.0.begin,
                                             c.0.decode.context),
                1);
            let pkt: Unknown<u8> = Unknown::from(ukn);
            assert_eq!(pkt.data().unwrap(), 20);
        }
    }

    fn check_callback<T>(cfg: &mut Config<T>, expect: T, expect_sz: i32) -> bool
        where T: PartialEq {
        unsafe {
            let mut ukn: pt_packet_unknown = std::mem::zeroed();
            return
                cfg.0.decode.callback.unwrap()(&mut ukn,
                                               cfg.0.as_ref(), cfg.0.begin,
                                               cfg.0.decode.context)
                == expect_sz
                && Unknown::<T>::from(ukn).data().unwrap() == expect;
        }
    }

    #[test]
    fn test_config_callback_safety() {
        let mut kektop = [10;9];
        let mut cfg = Config::new(&mut kektop);
        cfg.set_cpu(Cpu::intel(1, 2, 3));
        cfg.set_callback(|c, p,| {
            (Unknown::new(c.0.cpu.stepping + p[8]), 17)
        });

        for _ in 0..10 { assert!(check_callback(&mut cfg, 13, 17)) }
        cfg.set_callback(|c, p| {
            (Unknown::new(c.0.cpu.model + p[0]), 1)
        });
        for _ in 0..10 { assert!(check_callback(&mut cfg, 12, 1)) }
    }

    #[test]
    #[should_panic]
    fn test_config_callback_out_of_bounds() {
        let mut kektop = [10;9];
        let mut cfg = Config::new(&mut kektop);
        let mut raw: *const pt_config = cfg.0.as_ref();
        cfg.set_cpu(Cpu::intel(1, 2, 3));
        cfg.set_callback(|c, p,| {
            // make sure no move or copy is done
            if let Cow::Owned(_) = c.0 { panic!("BUG!") }
            assert_eq!(c.0.as_ref() as *const _, raw);
            (Unknown::new(p[100]), 17)
        });
        unsafe {
            let mut ukn: pt_packet_unknown = std::mem::zeroed();
            cfg.0.decode.callback.unwrap()(&mut ukn,
                                           cfg.0.as_ref(), cfg.0.begin,
                                           cfg.0.decode.context);
        }
    }
}

unsafe extern "C" fn decode_callback<'a, F, C>(ukn: *mut pt_packet_unknown,
                                               cfg: *const pt_config,
                                               pos: *const u8,
                                               ctx: *mut c_void) -> c_int
    where F: FnMut(&Config<C>, &[u8]) -> (Unknown<C>, u32) {

    let sz = (*cfg).end as usize - pos as usize;
    let pos = std::slice::from_raw_parts(pos, sz);

    let c = ctx as *mut F;
    let c = &mut *c;

    let (res, bytes) = c(&(&*cfg).into(), pos);
    // TODO
    // REMEMBER TO CATCH THE BOX FROM THE DECODER
    (*ukn).priv_ = match res.0 {
        Some(r) => Box::into_raw(r) as *mut _,
        None => std::ptr::null_mut()
    };

    bytes as i32
}

/// A libipt configuration
pub struct Config<'a, C> (pub(crate) Cow<'a, pt_config>, PhantomData<C>);
impl<'a, C> Config<'a, C> {
    /// Initializes a Config instance with only a buffer.
    ///
    /// Chain this functions with the setter methods to provide the arguments you need
    pub fn new(buf: &'a mut [u8]) -> Self {
        let mut cfg: pt_config = unsafe { mem::zeroed() };
        cfg.size  = mem::size_of::<pt_config>();
        cfg.begin = buf.as_mut_ptr();
        cfg.end   = unsafe { buf.as_mut_ptr().offset(buf.len() as isize) };
        Config::<C>(Cow::Owned(cfg), PhantomData)
    }

    #[inline]
    fn ensure_owned(&mut self) -> &mut pt_config {
        match &mut self.0 {
            Cow::Borrowed(_) => unreachable!(),
            Cow::Owned(c) => c
        }
    }

    /// The cpu used for capturing the data.
    /// It's highly recommended to provide this information.
    /// Processor specific workarounds will be identified this way.
    #[inline]
    pub fn set_cpu(&mut self, cpu: Cpu) {
        let c = self.ensure_owned();
        c.cpu = cpu.0;
        c.errata = cpu.determine_errata();
    }

    /// Frequency values used for timing packets (mtc)
    #[inline]
    pub fn set_freq(&mut self, freq: Frequency) {
        let c = self.ensure_owned();
        c.mtc_freq = freq.mtc;
        c.nom_freq = freq.nom;
        c.cpuid_0x15_eax = freq.tsc;
        c.cpuid_0x15_ebx = freq.ctc;
    }

    /// Decoder specific flags
    #[inline]
    pub fn set_flags(&mut self, flags: impl Into<pt_conf_flags>) {
        self.ensure_owned().flags = flags.into()
    }

    /// Address filter configuration
    #[inline]
    pub fn set_filter(&mut self, filter: AddrFilter) {
        self.ensure_owned().addr_filter = filter.0
    }

    /// A callback for decoding unknown packets
    #[inline]
    pub fn set_callback<'b, F>(&mut self, mut cb: F)
        where F: FnMut(&Config<C>, &[u8]) -> (Unknown<C>, u32),
              F: 'a {
        let c = self.ensure_owned();
        c.decode.callback = Some(decode_callback::<F, C>);
        c.decode.context  = &mut cb as *mut _ as *mut c_void;
    }
}

impl<'a, C> From<&'a pt_config> for Config<'a, C> {
    fn from(cfg: &'a pt_config) -> Self {
        Config(Cow::Borrowed(cfg), PhantomData)
    }
}