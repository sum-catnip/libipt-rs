use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_7;

#[cfg(test)]
mod test {
    use super::*;
    use super::super::Payload;
    use std::mem;
    use libipt_sys::{ pt_event, pt_event_type_ptev_overflow };

    #[test]
    fn test_overflow_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_overflow;
        evt.variant.overflow = pt_event__bindgen_ty_1__bindgen_ty_7 {
            ip: 11
        };

        let payload: Payload = evt.into();
        match payload {
            Payload::Overflow(e) => {
                assert_eq!(e.ip(), 11);
            },
            _ => unreachable!("oof")
        }
    }
}

/// Trace overflow
#[derive(Clone, Copy, Debug)] 
pub struct Overflow(pub(super) pt_event__bindgen_ty_1__bindgen_ty_7);
impl Overflow {
    /// The address at which tracing resumes after overflow.
    ///
    /// This field is not valid, if ip_suppressed is set.
    /// In this case, the overflow resolved while tracing was disabled.
    pub fn ip(self) -> u64 { self.0.ip }
}