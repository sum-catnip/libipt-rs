use crate::error::{PtError, PtErrorCode};
use crate::event::Event;
use derive_more::Deref;
use libipt_sys::pt_event_type_ptev_stop;
use std::fmt::{Debug, Formatter};

#[derive(Clone, Copy, Deref)]
#[repr(transparent)]
pub struct Stop {
    pub(super) event: Event,
}

impl Debug for Stop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stop {{")?;
        self.fmt_common_fields(f)?;
        write!(f, "}}")
    }
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
