use super::cpu::Cpu;
use super::freqency::Frequency;
use super::filter::AddrFilter;
use crate::packet::Unknown;
use crate::error::{ PtError, PtErrorCode };

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
    use crate::packet::Unknown;

    #[test]
    #[should_panic]
    fn test_config_empty() {
        let c = ConfigBuilder::new(&mut [0; 0]).unwrap().finish();
        assert_eq!(c.0.begin, c.0.end);
        assert_eq!(c.0.size, mem::size_of::<pt_config>());
    }

    #[test]
    fn test_config_buf() {
        let mut data = [0; 16];
        let len = data.len();
        let c = ConfigBuilder::new(&mut data).unwrap().finish();
        assert_eq!(c.0.end as usize - c.0.begin as usize, len);
    }

    #[test]
    fn test_config_all() {
        let mut data = [18; 3];
        let c = ConfigBuilder::with_callback(
            &mut data, |c, p| {
                (Unknown::new(c.0.cpu.model + p[0]), 1) })
            .unwrap()
            .filter(AddrFilterBuilder::new()
                .addr0(AddrRange::new(1, 2, AddrConfig::STOP))
                .addr1(AddrRange::new(3, 4, AddrConfig::FILTER))
                .addr2(AddrRange::new(5, 6, AddrConfig::DISABLED))
                .addr3(AddrRange::new(7, 8, AddrConfig::STOP))
                .finish())
            .cpu(Cpu::intel(1, 2, 3))
            .freq(Frequency::new(1, 2, 3, 4))
            .flags(BlockFlags::END_ON_CALL | BlockFlags::END_ON_JUMP)
            .finish();

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
        let mut cfg = ConfigBuilder::with_callback(
            &mut kektop,
            |c, p,| { (Unknown::new(c.0.cpu.stepping + p[8]), 17) })
            .unwrap()
            .cpu(Cpu::intel(1, 2, 3))
            .finish();

        for _ in 0..10 { assert!(check_callback(&mut cfg, 13, 17)) }
    }

    // FIXME
    #[ignore]
    #[test]
    #[should_panic]
    fn test_config_callback_out_of_bounds() {
        let mut kektop = [10;9];
        let cfg = ConfigBuilder::with_callback(&mut kektop, |c, p,| {
            // make sure no move or copy is done
            if let Cow::Owned(_) = c.0 { panic!("BUG!") }
            // assert_eq!(c.0.as_ref() as *const _, raw);
            (Unknown::new(p[100]), 17)
        }).unwrap().cpu(Cpu::intel(1, 2, 3)).finish();

        unsafe {
            let mut ukn: pt_packet_unknown = std::mem::zeroed();
            cfg.0.decode.callback.unwrap()(&mut ukn,
                                           cfg.0.as_ref(), cfg.0.begin,
                                           cfg.0.decode.context);
        }
    }

    #[test]
    fn test_builder_buf_lifetimes() {
        let mut x = [10; 10];
        let a : Config<()>;
        {
            let mut c = ConfigBuilder::new(&mut x).unwrap();
            a = c.finish();
            c.cpu(Cpu::intel(1, 2, 3));
            let b = c.finish();
            unsafe { assert_eq!(b.buffer(), [10; 10]) };
        }
        unsafe { assert_eq!(a.buffer(), [10; 10]) };
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
    (*ukn).priv_ = match res.0 {
        Some(r) => Box::into_raw(r) as *mut _,
        None => std::ptr::null_mut()
    };

    bytes as i32
}

/// A helper type to create the libipt Configuration instance
pub struct ConfigBuilder<'a, T> (pt_config, PhantomData<&'a mut T>);
impl<'a, T> ConfigBuilder<'a, T> {
    // when theres a bug here, there might be on in `new` too.
    /// Initializes a Config instance with a buffer and decoder callback
    pub fn with_callback<F>(buf: &'a mut [u8], mut cb: F) -> Result<Self, PtError>
        where F: FnMut(&Config<T>, &[u8]) -> (Unknown<T>, u32),
              F: 'a {
        // yeah.. libipt doesnt handle this -_-
        if buf.is_empty() { return Err(
            PtError::new(PtErrorCode::Invalid, "buffer cant be empty!")
        )}
        let mut cfg: pt_config = unsafe { mem::zeroed() };
        cfg.size  = mem::size_of::<pt_config>();
        cfg.begin = buf.as_mut_ptr();
        cfg.end   = unsafe { buf.as_mut_ptr().add(buf.len()) };
        cfg.decode.callback = Some(decode_callback::<F, T>);
        cfg.decode.context  = &mut cb as *mut _ as *mut c_void;
        Ok(ConfigBuilder::<T>(cfg, PhantomData))
    }

    /// The cpu used for capturing the data.
    /// It's highly recommended to provide this information.
    /// Processor specific workarounds will be identified this way.
    pub fn cpu(&mut self, cpu: Cpu) -> &mut Self {
        self.0.cpu = cpu.0;
        self.0.errata = cpu.determine_errata();

        self
    }

    /// Frequency values used for timing packets (mtc)
    pub fn freq(&mut self, freq: Frequency) -> &mut Self {
        self.0.mtc_freq = freq.mtc;
        self.0.nom_freq = freq.nom;
        self.0.cpuid_0x15_eax = freq.tsc;
        self.0.cpuid_0x15_ebx = freq.ctc;

        self
    }

    /// Decoder specific flags
    pub fn flags(&mut self, flags: impl Into<pt_conf_flags>) -> &mut Self {
        self.0.flags = flags.into();
        self
    }

    /// Address filter configuration
    pub fn filter(&mut self, filter: AddrFilter) -> &mut Self {
        self.0.addr_filter = filter.0;
        self
    }

    /// turn itself into a new `Config`
    pub fn finish(&self) -> Config<'a, T> {
        Config(Cow::Owned(self.0), self.1)
    }
}

impl<'a> ConfigBuilder<'a, ()> {
    /// Initializes a Config instance with only a buffer.
    /// If you want to use a decoder callback,
    /// use the `with_callback` function
    /// returns `Invalid` when buf is empty
    pub fn new(buf: &'a mut [u8]) -> Result<ConfigBuilder<'a, ()>, PtError> {
        if buf.is_empty() { return Err(
            PtError::new(PtErrorCode::Invalid, "buffer cant be empty!")
        )}
        let mut cfg: pt_config = unsafe { mem::zeroed() };
        cfg.size  = mem::size_of::<pt_config>();
        cfg.begin = buf.as_mut_ptr();
        cfg.end   = unsafe { buf.as_mut_ptr().add(buf.len()) };
        Ok(ConfigBuilder::<()>(cfg, PhantomData))
    }
}

/// A libipt configuration
pub struct Config<'a, C> (pub(crate) Cow<'a, pt_config>, PhantomData<&'a mut C>);
impl<'a, C> Config<'a, C> {
    /// Gets this configs buffer.
    /// This operation is unsafe because an encoder might write into the buffer
    /// at any time
    pub unsafe fn buffer(&self) -> &'a [u8] {
        std::slice::from_raw_parts(
            self.0.begin,
            self.0.end as usize - self.0.begin as usize
        )
    }
}

impl<'a, C> From<&'a pt_config> for Config<'a, C> {
    fn from(cfg: &'a pt_config) -> Self {
        Config(Cow::Borrowed(cfg), PhantomData)
    }
}
