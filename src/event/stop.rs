use crate::error::{PtError, PtErrorCode};
use crate::event::Event;
use derive_more::Deref;
use libipt_sys::pt_event_type_ptev_stop;

#[derive(Clone, Copy, Debug, Deref)]
#[repr(transparent)]
pub struct Stop {
    pub(super) event: Event,
}

impl TryFrom<Event> for Stop {
    type Error = PtError;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        if event.0.type_ == pt_event_type_ptev_stop {
            Ok(Self { event })
        } else {
            Err(PtErrorCode::Invalid.into())
        }
    }
}
