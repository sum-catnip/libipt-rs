use crate::error::{PtError, PtErrorCode};
use crate::event::Event;
use derive_more::Deref;
use libipt_sys::pt_event_type_ptev_tsx;
use std::fmt::{Debug, Formatter};

/// A transactional execution state change
#[derive(Clone, Copy, Deref)]
#[repr(transparent)]
pub struct Tsx {
    pub(super) event: Event,
}
impl Tsx {
    /// The address at which the event is effective.
    ///
    /// This field is not valid if @ip_suppressed is set.
    #[must_use]
    pub const fn ip(&self) -> u64 {
        unsafe { self.event.0.variant.tsx.ip }
    }

    /// A flag indicating speculative execution mode
    #[must_use]
    pub fn speculative(&self) -> bool {
        (unsafe { self.event.0.variant.tsx.speculative() }) > 0
    }

    /// A flag indicating speculative execution aborts
    #[must_use]
    pub fn aborted(&self) -> bool {
        (unsafe { self.event.0.variant.tsx.aborted() }) > 0
    }
}

impl Debug for Tsx {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tsx {{")?;
        self.fmt_common_fields(f)?;
        write!(
            f,
            "aborted: {:?}, ip: 0x{:x?}, speculative: {:?}, }}",
            self.aborted(),
            self.ip(),
            self.speculative()
        )
    }
}

impl TryFrom<Event> for Tsx {
    type Error = PtError;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        if event.0.type_ == pt_event_type_ptev_tsx {
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
    use libipt_sys::{pt_event, pt_event_type_ptev_tsx};
    use std::mem;

    #[test]
    fn test_tsx_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_tsx;
        evt.variant.tsx.ip = 11;
        unsafe {
            evt.variant.tsx.set_speculative(1);
        }

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::Tsx(e) => {
                assert_eq!(e.ip(), 11);
                assert!(e.speculative());
                assert!(!e.aborted());
            }
            _ => unreachable!("oof"),
        }
    }
}
