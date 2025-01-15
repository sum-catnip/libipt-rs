use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_18;

/// A core:bus ratio event
#[derive(Clone, Copy, Debug)]
pub struct Cbr(pub(super) pt_event__bindgen_ty_1__bindgen_ty_18);
impl Cbr {
    /// The core:bus ratio.
    #[must_use]
    pub fn ratio(self) -> u16 {
        self.0.ratio
    }
}

#[cfg(test)]
mod test {
    use super::super::Payload;
    use super::*;
    use libipt_sys::{pt_event, pt_event_type_ptev_cbr};
    use std::mem;

    #[test]
    fn test_cbr_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_cbr;
        evt.variant.cbr = pt_event__bindgen_ty_1__bindgen_ty_18 { ratio: 18 };

        let payload: Payload = evt.into();
        match payload {
            Payload::Cbr(e) => {
                assert_eq!(e.ratio(), 18);
            }
            _ => unreachable!("oof"),
        }
    }
}
