use crate::error::{PtError, PtErrorCode};
use crate::event::Event;
use derive_more::Deref;
use libipt_sys::pt_event_type_ptev_cbr;

/// A core:bus ratio event
#[derive(Clone, Copy, Debug, Deref)]
#[repr(transparent)]
pub struct Cbr {
    pub(super) event: Event,
}
impl Cbr {
    /// The core:bus ratio.
    #[must_use]
    pub const fn ratio(&self) -> u16 {
        unsafe { self.event.0.variant.cbr.ratio }
    }
}

impl TryFrom<Event> for Cbr {
    type Error = PtError;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        if event.0.type_ == pt_event_type_ptev_cbr {
            Ok(Self { event })
        } else {
            Err(PtErrorCode::Invalid.into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::EventType;
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_cbr};
    use std::mem;

    #[test]
    fn test_cbr_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_cbr;
        evt.variant.cbr.ratio = 18;

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::Cbr(e) => {
                assert_eq!(e.ratio(), 18);
            }
            _ => unreachable!("oof"),
        }
    }
}
