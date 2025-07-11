use crate::error::{PtError, PtErrorCode};
use crate::event::Event;
use derive_more::Deref;
use libipt_sys::pt_event_type_ptev_pwrx;

/// A power state was exited
#[derive(Clone, Copy, Debug, Deref)]
#[repr(transparent)]
pub struct Pwrx {
    pub(super) event: Event,
}
impl Pwrx {
    /// The core C-state at the time of the wake.
    #[must_use]
    pub const fn last(&self) -> u8 {
        unsafe { self.event.0.variant.pwrx.last }
    }

    /// The deepest core C-state achieved during sleep.
    #[must_use]
    pub const fn deepest(&self) -> u8 {
        unsafe { self.event.0.variant.pwrx.deepest }
    }

    /// The wake reason:
    ///
    /// - due to external interrupt received.
    #[must_use]
    pub fn interrupt(&self) -> bool {
        (unsafe { self.event.0.variant.pwrx.interrupt() }) > 0
    }

    /// The wake reason:
    ///
    /// - due to store to monitored address.
    #[must_use]
    pub fn store(&self) -> bool {
        (unsafe { self.event.0.variant.pwrx.store() }) > 0
    }

    /// The wake reason:
    ///
    /// - due to h/w autonomous condition such as HDC.
    #[must_use]
    pub fn autonomous(&self) -> bool {
        (unsafe { self.event.0.variant.pwrx.autonomous() }) > 0
    }
}

impl TryFrom<Event> for Pwrx {
    type Error = PtError;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        if event.0.type_ == pt_event_type_ptev_pwrx {
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
    use libipt_sys::{pt_event, pt_event_type_ptev_pwrx};
    use std::mem;

    #[test]
    fn test_pwrx_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_pwrx;
        evt.variant.pwrx.last = 11;
        evt.variant.pwrx.deepest = 22;
        unsafe {
            evt.variant.pwrx.set_interrupt(1);
            evt.variant.pwrx.set_autonomous(1);
        }

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::Pwrx(e) => {
                assert_eq!(e.last(), 11);
                assert_eq!(e.deepest(), 22);
                assert!(e.interrupt());
                assert!(!e.store());
                assert!(e.autonomous());
            }
            _ => unreachable!("oof"),
        }
    }
}
