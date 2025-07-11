use crate::error::{PtError, PtErrorCode};
use crate::event::Event;
use derive_more::Deref;
use libipt_sys::{pt_event_type_ptev_async_paging, pt_event_type_ptev_paging};

/// A synchronous paging event
#[derive(Clone, Copy, Debug, Deref)]
#[repr(transparent)]
pub struct Paging {
    pub(super) event: Event,
}
impl Paging {
    /// The updated CR3 value.
    /// The lower 5 bit have been zeroed out.
    /// The upper bits have been zeroed out depending on the maximum possible address.
    #[must_use]
    pub const fn cr3(&self) -> u64 {
        unsafe { self.event.0.variant.paging.cr3 }
    }

    /// A flag indicating whether the cpu is operating in
    /// vmx non-root (guest) mode.
    #[must_use]
    pub fn non_root(&self) -> bool {
        (unsafe { self.event.0.variant.paging.non_root() }) > 0
    }
}

impl TryFrom<Event> for Paging {
    type Error = PtError;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        if event.0.type_ == pt_event_type_ptev_paging {
            Ok(Self { event })
        } else {
            Err(PtErrorCode::Invalid.into())
        }
    }
}

/// An asynchronous paging event
#[derive(Clone, Copy, Debug)]
pub struct AsyncPaging {
    pub(super) event: Event,
}
impl AsyncPaging {
    /// The updated CR3 value.
    ///
    /// The lower 5 bit have been zeroed out.
    /// The upper bits have been zeroed out depending on the
    /// maximum possible address.
    #[must_use]
    pub const fn cr3(&self) -> u64 {
        unsafe { self.event.0.variant.async_paging.cr3 }
    }

    /// A flag indicating whether the cpu is operating in
    /// vmx non-root (guest) mode.
    #[must_use]
    pub fn non_root(&self) -> bool {
        (unsafe { self.event.0.variant.async_paging.non_root() }) > 0
    }

    /// The address at which the event is effective
    #[must_use]
    pub const fn ip(&self) -> u64 {
        unsafe { self.event.0.variant.async_paging.ip }
    }
}

impl TryFrom<Event> for AsyncPaging {
    type Error = PtError;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        if event.0.type_ == pt_event_type_ptev_async_paging {
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
    use libipt_sys::{pt_event, pt_event_type_ptev_async_paging, pt_event_type_ptev_paging};
    use std::mem;

    #[test]
    fn test_paging_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_paging;
        evt.variant.paging.cr3 = 11;
        unsafe {
            evt.variant.async_paging.set_non_root(1);
        }

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::Paging(e) => {
                assert_eq!(e.cr3(), 11);
                assert!(e.non_root());
            }
            _ => unreachable!("oof"),
        }
    }

    #[test]
    fn test_async_paging_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_async_paging;

        evt.variant.async_paging.cr3 = 11;
        evt.variant.async_paging.ip = 12;
        unsafe {
            evt.variant.async_paging.set_non_root(1);
        }

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::AsyncPaging(e) => {
                assert_eq!(e.cr3(), 11);
                assert_eq!(e.ip(), 12);
                assert!(e.non_root());
            }
            _ => unreachable!("oof"),
        }
    }
}
