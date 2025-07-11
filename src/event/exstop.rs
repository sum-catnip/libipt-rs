use crate::error::{PtError, PtErrorCode};
use crate::event::Event;
use derive_more::Deref;
use libipt_sys::pt_event_type_ptev_exstop;

/// Execution has stopped
#[derive(Clone, Copy, Debug, Deref)]
#[repr(transparent)]
pub struct Exstop {
    pub(super) event: Event,
}
impl Exstop {
    /// The address at which execution has stopped. This is the last instruction that did not complete.
    ///
    /// This field is not valid, if @ip_suppressed is set.
    #[must_use]
    pub const fn ip(&self) -> u64 {
        unsafe { self.event.0.variant.exstop.ip }
    }
}

impl TryFrom<Event> for Exstop {
    type Error = PtError;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        if event.0.type_ == pt_event_type_ptev_exstop {
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
    use libipt_sys::{pt_event, pt_event_type_ptev_exstop};
    use std::mem;

    #[test]
    fn test_exstop_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_exstop;
        evt.variant.exstop.ip = 11;

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::Exstop(e) => {
                assert_eq!(e.ip(), 11);
            }
            _ => unreachable!("oof"),
        }
    }
}
