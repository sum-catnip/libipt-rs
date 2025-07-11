use crate::error::{PtError, PtErrorCode};
use crate::event::Event;
use derive_more::Deref;
use libipt_sys::{pt_event_type_ptev_async_vmcs, pt_event_type_ptev_vmcs};

/// A synchronous vmcs event
#[derive(Clone, Copy, Debug, Deref)]
#[repr(transparent)]
pub struct Vmcs {
    pub(super) event: Event,
}
impl Vmcs {
    /// The VMCS base address.
    ///
    /// The address is zero-extended with the lower 12 bits all zero
    #[must_use]
    pub const fn base(&self) -> u64 {
        unsafe { self.event.0.variant.vmcs.base }
    }
}

impl TryFrom<Event> for Vmcs {
    type Error = PtError;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        if event.0.type_ == pt_event_type_ptev_vmcs {
            Ok(Self { event })
        } else {
            Err(PtErrorCode::Invalid.into())
        }
    }
}

/// An asynchronous vmcs event
#[derive(Clone, Copy, Debug, Deref)]
pub struct AsyncVmcs {
    pub(super) event: Event,
}
impl AsyncVmcs {
    /// The VMCS base address.
    ///
    /// The address is zero-extended with the lower 12 bits all zero
    #[must_use]
    pub const fn base(&self) -> u64 {
        unsafe { self.event.0.variant.async_vmcs.base }
    }

    /// The address at which the event is effective.
    #[must_use]
    pub const fn ip(&self) -> u64 {
        unsafe { self.event.0.variant.async_vmcs.ip }
    }
}

impl TryFrom<Event> for AsyncVmcs {
    type Error = PtError;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        if event.0.type_ == pt_event_type_ptev_async_vmcs {
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
    use libipt_sys::{pt_event, pt_event_type_ptev_async_vmcs, pt_event_type_ptev_vmcs};
    use std::mem;

    #[test]
    fn test_vmcs_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_vmcs;
        evt.variant.vmcs.base = 11;

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::Vmcs(e) => {
                assert_eq!(e.base(), 11);
            }
            _ => unreachable!("oof"),
        }
    }

    #[test]
    fn test_async_vmcs_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_async_vmcs;
        evt.variant.async_vmcs.base = 11;
        evt.variant.async_vmcs.ip = 12;

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::AsyncVmcs(e) => {
                assert_eq!(e.base(), 11);
                assert_eq!(e.ip(), 12);
            }
            _ => unreachable!("oof"),
        }
    }
}
