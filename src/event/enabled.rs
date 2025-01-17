use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_1;

/// Tracing has been enabled
#[derive(Clone, Copy, Debug)]
pub struct Enabled(pub(super) pt_event__bindgen_ty_1__bindgen_ty_1);
impl Enabled {
    /// The address at which tracing resumes
    #[must_use]
    pub fn ip(&self) -> u64 {
        self.0.ip
    }

    /// A flag indicating that tracing resumes from the IP
    /// at which tracing had been disabled before.
    #[must_use]
    pub fn resumed(&self) -> bool {
        self.0.resumed() > 0
    }
}

#[cfg(test)]
mod test {
    use super::super::Payload;
    use super::*;
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_enabled};
    use std::mem;

    #[test]
    fn test_enabled_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_enabled;
        evt.variant.enabled = pt_event__bindgen_ty_1__bindgen_ty_1 {
            ip: 11,
            _bitfield_align_1: [],
            _bitfield_1: pt_event__bindgen_ty_1__bindgen_ty_1::new_bitfield_1(1),
            __bindgen_padding_0: Default::default(),
        };

        let payload: Payload = Event(evt).into();
        match payload {
            Payload::Enabled(e) => {
                assert_eq!(e.ip(), 11);
                assert!(e.resumed())
            }
            _ => unreachable!("oof"),
        }
    }
}
