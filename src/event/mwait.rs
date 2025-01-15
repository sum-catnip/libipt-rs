use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_13;

/// An MWAIT operation completed
#[derive(Clone, Copy, Debug)]
pub struct Mwait(pub(super) pt_event__bindgen_ty_1__bindgen_ty_13);
impl Mwait {
    /// The address of the instruction causing the mwait.
    ///
    /// This field is not valid, if @ip_suppressed is set.
    #[must_use]
    pub fn ip(self) -> u64 {
        self.0.ip
    }

    /// The mwait hints (eax).
    ///
    /// Reserved bits are undefined.
    #[must_use]
    pub fn hints(self) -> u32 {
        self.0.hints
    }

    /// The mwait extensions (ecx).
    ///
    /// Reserved bits are undefined.
    #[must_use]
    pub fn ext(self) -> u32 {
        self.0.ext
    }
}

#[cfg(test)]
mod test {
    use super::super::Payload;
    use super::*;
    use libipt_sys::{pt_event, pt_event_type_ptev_mwait};
    use std::mem;

    #[test]
    fn test_mwait_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_mwait;
        evt.variant.mwait = pt_event__bindgen_ty_1__bindgen_ty_13 {
            ip: 11,
            hints: 22,
            ext: 33,
        };

        let payload: Payload = evt.into();
        match payload {
            Payload::Mwait(e) => {
                assert_eq!(e.ip(), 11);
                assert_eq!(e.hints(), 22);
                assert_eq!(e.ext(), 33);
            }
            _ => unreachable!("oof"),
        }
    }
}
