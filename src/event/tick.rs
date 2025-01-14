use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_17;

/// A timing event
#[derive(Clone, Copy, Debug)]
pub struct Tick(pub(super) pt_event__bindgen_ty_1__bindgen_ty_17);
impl Tick {
    /// The instruction address near which the tick occured.
    ///
    /// A timestamp can sometimes be attributed directly to
    /// an instruction (e.g. to an indirect branch that
    /// receives CYC + TIP) and sometimes not (e.g. MTC).
    ///
    /// This field is not valid, if \@ip_suppressed is set.
    pub fn ip(self) -> u64 {
        self.0.ip
    }
}

#[cfg(test)]
mod test {
    use super::super::Payload;
    use super::*;
    use libipt_sys::{pt_event, pt_event_type_ptev_tick};
    use std::mem;

    #[test]
    fn test_tick_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_tick;
        evt.variant.tick = pt_event__bindgen_ty_1__bindgen_ty_17 { ip: 11 };

        let payload: Payload = evt.into();
        match payload {
            Payload::Tick(e) => {
                assert_eq!(e.ip(), 11);
            }
            _ => unreachable!("oof"),
        }
    }
}
