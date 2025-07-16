use crate::error::{PtError, PtErrorCode};
use crate::event::Event;
use derive_more::Deref;
use libipt_sys::pt_event_type_ptev_overflow;
use std::fmt::{Debug, Formatter};

/// Trace overflow
#[derive(Clone, Copy, Deref)]
#[repr(transparent)]
pub struct Overflow {
    pub(super) event: Event,
}
impl Overflow {
    /// The address at which tracing resumes after overflow.
    ///
    /// This field is not valid, if ip_suppressed is set.
    /// In this case, the overflow resolved while tracing was disabled.
    pub const fn ip(&self) -> u64 {
        unsafe { self.event.0.variant.overflow.ip }
    }
}

impl Debug for Overflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Overflow {{")?;
        self.fmt_common_fields(f)?;
        write!(f, "ip: 0x{:x?}, }}", self.ip())
    }
}

impl TryFrom<Event> for Overflow {
    type Error = PtError;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        if event.0.type_ == pt_event_type_ptev_overflow {
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
    use libipt_sys::{pt_event, pt_event_type_ptev_overflow};
    use std::mem;

    #[test]
    fn test_overflow_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_overflow;
        evt.variant.overflow.ip = 11;

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::Overflow(e) => {
                assert_eq!(e.ip(), 11);
            }
            _ => unreachable!("oof"),
        }
    }
}
