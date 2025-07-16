// Certain casts are required only on Windows. Inform Clippy to ignore them.
#![allow(clippy::unnecessary_cast)]

use crate::error::{PtError, PtErrorCode};
use crate::event::Event;
use derive_more::Deref;
use libipt_sys::{
    pt_event_type_ptev_exec_mode, pt_exec_mode_ptem_16bit, pt_exec_mode_ptem_32bit,
    pt_exec_mode_ptem_64bit, pt_exec_mode_ptem_unknown,
};
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};

#[derive(Clone, Copy, TryFromPrimitive, Debug, PartialEq)]
#[repr(u32)]
pub enum ExecModeType {
    Bit16 = pt_exec_mode_ptem_16bit as u32,
    Bit32 = pt_exec_mode_ptem_32bit as u32,
    Bit64 = pt_exec_mode_ptem_64bit as u32,
    Unknown = pt_exec_mode_ptem_unknown as u32,
}

/// An execution mode change
#[derive(Clone, Copy, Deref)]
#[repr(transparent)]
pub struct ExecMode {
    pub(super) event: Event,
}
impl ExecMode {
    /// The address at which the event is effective
    #[must_use]
    pub const fn ip(&self) -> u64 {
        unsafe { self.event.0.variant.exec_mode.ip }
    }

    /// The execution mode
    #[must_use]
    pub fn mode(&self) -> ExecModeType {
        ExecModeType::try_from(unsafe { self.event.0.variant.exec_mode.mode } as u32).unwrap()
    }
}

impl Debug for ExecMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExecMode {{")?;
        self.fmt_common_fields(f)?;
        write!(f, "ip: 0x{:x?}, mode: {:?} }}", self.ip(), self.mode())
    }
}

impl TryFrom<Event> for ExecMode {
    type Error = PtError;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        if event.0.type_ == pt_event_type_ptev_exec_mode {
            Ok(Self { event })
        } else {
            Err(PtErrorCode::Invalid.into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::EventType;
    use super::*;
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_exec_mode};
    use std::mem;

    #[test]
    fn test_exec_mode_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_exec_mode;

        evt.variant.exec_mode.ip = 11;
        evt.variant.exec_mode.mode = pt_exec_mode_ptem_32bit;

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::ExecMode(e) => {
                assert_eq!(e.ip(), 11);
                assert_eq!(e.mode(), ExecModeType::Bit32);
            }
            _ => unreachable!("oof"),
        }
    }
}
