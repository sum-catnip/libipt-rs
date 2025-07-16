use crate::error::{PtError, PtErrorCode};
use crate::event::Event;
use derive_more::Deref;
use libipt_sys::pt_event_type_ptev_async_branch;
use std::fmt::{Debug, Formatter};

/// An asynchronous branch, e.g. interrupt
#[derive(Clone, Copy, Deref)]
#[repr(transparent)]
pub struct AsyncBranch {
    pub(super) event: Event,
}

impl AsyncBranch {
    /// The branch source address
    #[must_use]
    pub const fn from(&self) -> u64 {
        unsafe { self.event.0.variant.async_branch.from }
    }

    /// The branch destination address.
    /// This field is not valid if @ip_suppressed is set.
    #[must_use]
    pub const fn to(&self) -> u64 {
        unsafe { self.event.0.variant.async_branch.to }
    }
}

impl Debug for AsyncBranch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AsyncBranch {{")?;
        self.fmt_common_fields(f)?;
        write!(f, "from: 0x{:x?}, to: 0x{:x?}, }}", self.from(), self.to())
    }
}

impl TryFrom<Event> for AsyncBranch {
    type Error = PtError;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        if event.0.type_ == pt_event_type_ptev_async_branch {
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
    use libipt_sys::{pt_event, pt_event_type_ptev_async_branch};
    use std::mem;

    #[test]
    fn test_branch_async_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_async_branch;
        evt.variant.async_branch.from = 1;
        evt.variant.async_branch.to = 2;

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::AsyncBranch(e) => {
                assert_eq!(e.from(), 1);
                assert_eq!(e.to(), 2);
            }
            _ => unreachable!("oof"),
        }
    }
}
