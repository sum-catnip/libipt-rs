use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_15;

#[cfg(test)]
mod test {
    use super::*;
    use super::super::Payload;
    use std::mem;
    use libipt_sys::{ pt_event, pt_event_type_ptev_pwrx };

    #[test]
    fn test_pwrx_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_pwrx;
        evt.variant.pwrx = pt_event__bindgen_ty_1__bindgen_ty_15 {
            last: 11,
            deepest: 22,
            _bitfield_1: pt_event__bindgen_ty_1__bindgen_ty_15::new_bitfield_1(1, 0, 1),
            __bindgen_padding_0: Default::default()
        };

        let payload: Payload = evt.into();
        match payload {
            Payload::Pwrx(e) => {
                assert_eq!(e.last(), 11);
                assert_eq!(e.deepest(), 22);
                assert!(e.interrupt());
                assert!(!e.store());
                assert!(e.autonomous());
            },
            _ => unreachable!("oof")
        }
    }
}

/// A power state was exited
#[derive(Clone, Copy, Debug)]
pub struct Pwrx(pub(super) pt_event__bindgen_ty_1__bindgen_ty_15);
impl Pwrx {
    /// The core C-state at the time of the wake.
    pub fn last(self) -> u8 { self.0.last }
    /// The deepest core C-state achieved during sleep.
    pub fn deepest(self) -> u8 { self.0.deepest }
    /// The wake reason:
    ///
    /// - due to external interrupt received.
    pub fn interrupt(self) -> bool { self.0.interrupt() > 0 }
    /// The wake reason:
    ///
    /// - due to store to monitored address.
    pub fn store(self) -> bool { self.0.store() > 0 }
    /// The wake reason:
    ///
    /// - due to h/w autonomous condition such as HDC.
    pub fn autonomous(self) -> bool { self.0.autonomous() > 0 }
}