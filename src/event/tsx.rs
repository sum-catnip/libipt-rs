use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_9;

/// A transactional execution state change
#[derive(Clone, Copy, Debug)]
pub struct Tsx(pub(super) pt_event__bindgen_ty_1__bindgen_ty_9);
impl Tsx {
    /// The address at which the event is effective.
    ///
    /// This field is not valid if @ip_suppressed is set.
    #[must_use]
    pub fn ip(&self) -> u64 {
        self.0.ip
    }

    /// A flag indicating speculative execution mode
    #[must_use]
    pub fn speculative(&self) -> bool {
        self.0.speculative() > 0
    }

    /// A flag indicating speculative execution aborts
    #[must_use]
    pub fn aborted(&self) -> bool {
        self.0.aborted() > 0
    }
}

#[cfg(test)]
mod test {
    use super::super::Payload;
    use super::*;
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_tsx};
    use std::mem;

    #[test]
    fn test_tsx_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_tsx;
        evt.variant.tsx = pt_event__bindgen_ty_1__bindgen_ty_9 {
            ip: 11,
            _bitfield_align_1: [],
            _bitfield_1: pt_event__bindgen_ty_1__bindgen_ty_9::new_bitfield_1(1, 0),
            __bindgen_padding_0: Default::default(),
        };

        let payload: Payload = Event(evt).into();
        match payload {
            Payload::Tsx(e) => {
                assert_eq!(e.ip(), 11);
                assert!(e.speculative());
                assert!(!e.aborted());
            }
            _ => unreachable!("oof"),
        }
    }
}
