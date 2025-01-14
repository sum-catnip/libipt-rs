use super::cpu::Cpu;
use super::filter::AddrFilter;
use super::freqency::Frequency;
use crate::error::{PtError, PtErrorCode};

use libipt_sys::{pt_conf_flags, pt_config};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem;

// unsafe extern "C" fn decode_callback<'a, F, C>(
//     ukn: *mut pt_packet_unknown,
//     cfg: *const pt_config,
//     pos: *const u8,
//     ctx: *mut c_void,
// ) -> c_int
// where
//     F: FnMut(&Config<C>, &[u8]) -> (Unknown<C>, u32),
// {
//     let sz = (*cfg).end as usize - pos as usize;
//     let pos = std::slice::from_raw_parts(pos, sz);
//
//     let c = ctx as *mut F;
//     let c = &mut *c;
//
//     let (res, bytes) = c(&(&*cfg).into(), pos);
//     (*ukn).priv_ = match res.0 {
//         Some(r) => Box::into_raw(r) as *mut _,
//         None => std::ptr::null_mut(),
//     };
//
//     bytes as i32
// }

pub trait PtEncoderDecoder {
    fn builder() -> EncoderDecoderBuilder<Self>
    where
        Self: Sized,
    {
        EncoderDecoderBuilder::default()
    }

    fn new_from_builder(builder: EncoderDecoderBuilder<Self>) -> Result<Self, PtError>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct EncoderDecoderBuilder<T> {
    pub(crate) config: pt_config,
    target: PhantomData<T>,
}

impl<T> Default for EncoderDecoderBuilder<T> {
    fn default() -> Self {
        let mut config: pt_config = unsafe { mem::zeroed() };
        config.size = size_of::<pt_config>();
        Self {
            config,
            target: PhantomData,
        }
    }
}

impl<T> EncoderDecoderBuilder<T>
where
    T: PtEncoderDecoder,
{
    /// Initializes an EncoderDecoderBuilder instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the encoder/decoder buffer from a raw pointer and length.
    /// The buffer is not copied.
    ///
    /// # Safety
    /// The buffer pointer `buf_ptr` must be valid for the entire encoder/decoder lifetime.
    pub unsafe fn buffer_from_raw(mut self, buf_ptr: *mut u8, buf_len: usize) -> Self {
        self.config.begin = buf_ptr;
        self.config.end = unsafe { buf_ptr.add(buf_len) };

        self
    }

    // TODO
    // /// Set a decoder callback.
    // pub fn callback<F>(&mut self, mut cb: F) -> Result<&mut Self, PtError>
    // where
    //     F: FnMut(&Config<T>, &[u8]) -> (Unknown<T>, u32),
    //     F: 'a,
    // {
    //     let mut cfg: pt_config = unsafe { mem::zeroed() };
    //     cfg.size = mem::size_of::<pt_config>();
    //     cfg.begin = buf.as_mut_ptr();
    //     cfg.end = unsafe { buf.as_mut_ptr().add(buf.len()) };
    //     cfg.decode.callback = Some(decode_callback::<F, T>);
    //     cfg.decode.context = &mut cb as *mut _ as *mut c_void;
    //     Ok(EncoderDecoderBuilder::<T>(cfg, PhantomData))
    // }

    /// The cpu used for capturing the data.
    /// It's highly recommended to provide this information.
    /// Processor specific workarounds will be identified this way.
    pub fn cpu(mut self, cpu: Cpu) -> Self {
        self.config.cpu = cpu.0;
        self.config.errata = cpu.determine_errata();

        self
    }

    /// Frequency values used for timing packets (mtc)
    pub fn freq(mut self, freq: Frequency) -> Self {
        self.config.mtc_freq = freq.mtc;
        self.config.nom_freq = freq.nom;
        self.config.cpuid_0x15_eax = freq.tsc;
        self.config.cpuid_0x15_ebx = freq.ctc;

        self
    }

    /// Decoder specific flags
    pub fn flags(mut self, flags: impl Into<pt_conf_flags>) -> Self {
        self.config.flags = flags.into();
        self
    }

    /// Address filter configuration
    pub fn filter(mut self, filter: AddrFilter) -> Self {
        self.config.addr_filter = filter.0;
        self
    }

    /// Turn itself into a PT encoder/decoder
    ///
    /// Returns `Err` if the buffer is not set.
    pub fn build(self) -> Result<T, PtError> {
        if self.config.begin.is_null() && self.config.end.is_null() {
            Err(PtError::new(
                PtErrorCode::BadConfig,
                "To build an encoder/decoder, a buffer must be set",
            ))
        } else {
            T::new_from_builder(self)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::*;

    struct FooDecoder {}

    impl PtEncoderDecoder for FooDecoder {
        fn new_from_builder(_: EncoderDecoderBuilder<Self>) -> Result<Self, PtError> {
            Ok(Self {})
        }
    }

    #[test]
    #[should_panic]
    fn test_config_empty() {
        let c = EncoderDecoderBuilder::<FooDecoder>::new();
        assert_eq!(c.config.begin, c.config.end);
        assert_eq!(c.config.size, size_of::<pt_config>());
        c.build().unwrap();
    }

    #[test]
    fn test_config_buf() {
        let mut data = [0u8; 16];
        let len = data.len();
        let mut c = EncoderDecoderBuilder::<FooDecoder>::new();
        c = unsafe { c.buffer_from_raw(data.as_mut_ptr(), data.len()) };
        assert_eq!(c.config.end as usize - c.config.begin as usize, len);
    }

    #[test]
    fn test_config_all() {
        let mut data = [18u8; 3];
        let mut c = EncoderDecoderBuilder::<FooDecoder>::new()
            .filter(
                AddrFilterBuilder::new()
                    .addr0(AddrRange::new(1, 2, AddrConfig::STOP))
                    .addr1(AddrRange::new(3, 4, AddrConfig::FILTER))
                    .addr2(AddrRange::new(5, 6, AddrConfig::DISABLED))
                    .addr3(AddrRange::new(7, 8, AddrConfig::STOP))
                    .finish(),
            )
            .cpu(Cpu::intel(1, 2, 3))
            .freq(Frequency::new(1, 2, 3, 4))
            .flags(BlockFlags::END_ON_CALL | BlockFlags::END_ON_JUMP);
        c = unsafe { c.buffer_from_raw(data.as_mut_ptr(), data.len()) };

        assert_eq!(c.config.cpu.family, 1);
        assert_eq!(c.config.cpu.model, 2);
        assert_eq!(c.config.cpu.stepping, 3);

        assert_eq!(c.config.mtc_freq, 1);
        assert_eq!(c.config.nom_freq, 2);
        assert_eq!(c.config.cpuid_0x15_ebx, 3);
        assert_eq!(c.config.cpuid_0x15_eax, 4);

        unsafe {
            assert_eq!(c.config.flags.variant.block.end_on_call(), 1);
            assert_eq!(c.config.flags.variant.block.end_on_jump(), 1);
            assert_eq!(c.config.flags.variant.block.enable_tick_events(), 0);
            assert_eq!(c.config.flags.variant.block.keep_tcal_on_ovf(), 0);
        }

        assert_eq!(c.config.addr_filter.addr0_a, 1);
        assert_eq!(c.config.addr_filter.addr0_b, 2);
        assert_eq!(
            unsafe { c.config.addr_filter.config.ctl.addr0_cfg() },
            AddrConfig::STOP as u32
        );

        assert_eq!(c.config.addr_filter.addr1_a, 3);
        assert_eq!(c.config.addr_filter.addr1_b, 4);
        assert_eq!(
            unsafe { c.config.addr_filter.config.ctl.addr1_cfg() },
            AddrConfig::FILTER as u32
        );

        assert_eq!(c.config.addr_filter.addr2_a, 5);
        assert_eq!(c.config.addr_filter.addr2_b, 6);
        assert_eq!(
            unsafe { c.config.addr_filter.config.ctl.addr2_cfg() },
            AddrConfig::DISABLED as u32
        );

        assert_eq!(c.config.addr_filter.addr3_a, 7);
        assert_eq!(c.config.addr_filter.addr3_b, 8);
        assert_eq!(
            unsafe { c.config.addr_filter.config.ctl.addr3_cfg() },
            AddrConfig::STOP as u32
        );

        // unsafe {
        //     let mut ukn: pt_packet_unknown = std::mem::zeroed();
        //     assert_eq!(
        //         c.config.decode.callback.unwrap()(
        //             &mut ukn,
        //             &raw const c.config,
        //             c.config.begin,
        //             c.config.decode.context
        //         ),
        //         1
        //     );
        //     let pkt: Unknown<u8> = Unknown::from(ukn);
        //     assert_eq!(pkt.data().unwrap(), 20);
        // }
    }

    // fn check_callback<T>(cfg: &mut Config<T>, expect: T, expect_sz: i32) -> bool
    // where
    //     T: PartialEq,
    // {
    //     unsafe {
    //         let mut ukn: pt_packet_unknown = std::mem::zeroed();
    //         return cfg.config.decode.callback.unwrap()(
    //             &mut ukn,
    //             cfg.config.as_ref(),
    //             cfg.config.begin,
    //             cfg.config.decode.context,
    //         ) == expect_sz
    //             && Unknown::<T>::from(ukn).data().unwrap() == expect;
    //     }
    // }

    // #[test]
    // fn test_config_callback_safety() {
    //     let mut kektop = [10; 9];
    //     let mut cfg = EncoderDecoderBuilder::with_callback(&mut kektop, |c, p| {
    //         (Unknown::new(c.config.cpu.stepping + p[8]), 17)
    //     })
    //     .unwrap()
    //     .cpu(Cpu::intel(1, 2, 3))
    //     .finish();
    //
    //     for _ in 0..10 {
    //         assert!(check_callback(&mut cfg, 13, 17))
    //     }
    // }

    // // FIXME
    // #[ignore]
    // #[test]
    // #[should_panic]
    // fn test_config_callback_out_of_bounds() {
    //     let mut kektop = [10; 9];
    //     let cfg = EncoderDecoderBuilder::with_callback(&mut kektop, |c, p| {
    //         // make sure no move or copy is done
    //         if let Cow::Owned(_) = c.0 {
    //             panic!("BUG!")
    //         }
    //         // assert_eq!(c.config.as_ref() as *const _, raw);
    //         (Unknown::new(p[100]), 17)
    //     })
    //     .unwrap()
    //     .cpu(Cpu::intel(1, 2, 3))
    //     .finish();
    //
    //     unsafe {
    //         let mut ukn: pt_packet_unknown = std::mem::zeroed();
    //         cfg.config.decode.callback.unwrap()(
    //             &mut ukn,
    //             cfg.config.as_ref(),
    //             cfg.config.begin,
    //             cfg.config.decode.context,
    //         );
    //     }
    // }
}
