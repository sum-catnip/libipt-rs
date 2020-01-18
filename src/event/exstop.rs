use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_12;

#[cfg(test)]
mod test {
    use super::*;
    use super::super::Payload;
    use std::mem;
    use libipt_sys::{ pt_event, pt_event_type_ptev_exstop };

    #[test]
    fn test_exstop_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_exstop;
        evt.variant.exstop = pt_event__bindgen_ty_1__bindgen_ty_12 {
            ip: 11,
        };

        let payload: Payload = evt.into();
        match payload {
            Payload::Exstop(e) => {
                assert_eq!(e.ip(), 11);
            },
            _ => unreachable!("oof")
        }
    }
}

/// Execution has stopped
#[derive(Clone, Copy, Debug)]
pub struct Exstop(pub(super) pt_event__bindgen_ty_1__bindgen_ty_12);
impl Exstop {
    /// The address at which execution has stopped. This is the last instruction that did not complete.
    ///
    /// This field is not valid, if @ip_suppressed is set.
    pub fn ip(self) -> u64 { self.0.ip }
}