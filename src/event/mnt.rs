use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_19;

/// A maintenance event.
#[derive(Clone, Copy, Debug)]
pub struct Mnt(pub(super) pt_event__bindgen_ty_1__bindgen_ty_19);
impl Mnt {
    /// The raw payload.
    #[must_use]
    pub fn payload(self) -> u64 {
        self.0.payload
    }
}

#[cfg(test)]
mod test {
    use super::super::Payload;
    use super::*;
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_mnt};
    use std::mem;

    #[test]
    fn test_mnt_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_mnt;
        evt.variant.mnt = pt_event__bindgen_ty_1__bindgen_ty_19 { payload: 17 };

        let payload: Payload = Event(evt).into();
        match payload {
            Payload::Mnt(e) => {
                assert_eq!(e.payload(), 17);
            }
            _ => unreachable!("oof"),
        }
    }
}
