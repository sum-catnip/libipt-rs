use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_14;

/// A power state was entered
#[derive(Clone, Copy, Debug)]
pub struct Pwre(pub(super) pt_event__bindgen_ty_1__bindgen_ty_14);
impl Pwre {
    /// The resolved thread C-state.
    #[must_use]
    pub fn state(self) -> u8 {
        self.0.state
    }

    /// The resolved thread sub C-state
    #[must_use]
    pub fn sub_state(self) -> u8 {
        self.0.sub_state
    }

    /// A flag indicating whether the C-state entry was
    /// initiated by h/w.
    #[must_use]
    pub fn hw(self) -> bool {
        self.0.hw() > 0
    }
}

#[cfg(test)]
mod test {
    use super::super::Payload;
    use super::*;
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_pwre};
    use std::mem;

    #[test]
    fn test_pwre_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_pwre;
        evt.variant.pwre = pt_event__bindgen_ty_1__bindgen_ty_14 {
            state: 11,
            sub_state: 22,
            _bitfield_align_1: [],
            _bitfield_1: pt_event__bindgen_ty_1__bindgen_ty_14::new_bitfield_1(1),
            __bindgen_padding_0: Default::default(),
        };

        let payload: Payload = Event(evt).into();
        match payload {
            Payload::Pwre(e) => {
                assert_eq!(e.state(), 11);
                assert_eq!(e.sub_state(), 22);
                assert!(e.hw())
            }
            _ => unreachable!("oof"),
        }
    }
}
