use crate::error::{PtError, PtErrorCode};
use crate::event::Event;
use derive_more::Deref;
use libipt_sys::pt_event_type_ptev_mwait;

/// An MWAIT operation completed
#[derive(Clone, Copy, Debug, Deref)]
#[repr(transparent)]
pub struct Mwait {
    pub(super) event: Event,
}
impl Mwait {
    /// The address of the instruction causing the mwait.
    ///
    /// This field is not valid, if @ip_suppressed is set.
    #[must_use]
    pub const fn ip(&self) -> u64 {
        unsafe { self.event.0.variant.mwait.ip }
    }

    /// The mwait hints (eax).
    ///
    /// Reserved bits are undefined.
    #[must_use]
    pub const fn hints(&self) -> u32 {
        unsafe { self.event.0.variant.mwait.hints }
    }

    /// The mwait extensions (ecx).
    ///
    /// Reserved bits are undefined.
    #[must_use]
    pub const fn ext(&self) -> u32 {
        unsafe { self.event.0.variant.mwait.ext }
    }
}

impl TryFrom<Event> for Mwait {
    type Error = PtError;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        if event.0.type_ == pt_event_type_ptev_mwait {
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
    use libipt_sys::{pt_event, pt_event_type_ptev_mwait};
    use std::mem;

    #[test]
    fn test_mwait_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_mwait;
        let mwait = unsafe { &mut evt.variant.mwait };
        mwait.ip = 11;
        mwait.hints = 22;
        mwait.ext = 33;

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::Mwait(e) => {
                assert_eq!(e.ip(), 11);
                assert_eq!(e.hints(), 22);
                assert_eq!(e.ext(), 33);
            }
            _ => unreachable!("oof"),
        }
    }
}
