// Certain casts are required only on Windows. Inform Clippy to ignore them.
#![allow(clippy::unnecessary_cast)]

use libipt_sys::{
    pt_event__bindgen_ty_1__bindgen_ty_8, pt_exec_mode_ptem_16bit, pt_exec_mode_ptem_32bit,
    pt_exec_mode_ptem_64bit, pt_exec_mode_ptem_unknown,
};
use std::convert::TryFrom;

use num_enum::TryFromPrimitive;

#[derive(Clone, Copy, TryFromPrimitive, Debug, PartialEq)]
#[repr(u32)]
pub enum ExecModeType {
    Bit16 = pt_exec_mode_ptem_16bit as u32,
    Bit32 = pt_exec_mode_ptem_32bit as u32,
    Bit64 = pt_exec_mode_ptem_64bit as u32,
    Unknown = pt_exec_mode_ptem_unknown as u32,
}

/// An execution mode change
#[derive(Clone, Copy, Debug)]
pub struct ExecMode(pub(super) pt_event__bindgen_ty_1__bindgen_ty_8);
impl ExecMode {
    /// The address at which the event is effective
    #[must_use]
    pub fn ip(&self) -> u64 {
        self.0.ip
    }

    /// The execution mode
    #[must_use]
    #[expect(clippy::missing_panics_doc)]
    pub fn mode(&self) -> ExecModeType {
        ExecModeType::try_from(self.0.mode as u32).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::super::Payload;
    use super::*;
    use libipt_sys::{pt_event, pt_event_type_ptev_exec_mode};
    use std::mem;

    #[test]
    fn test_exec_mode_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_exec_mode;
        evt.variant.exec_mode = pt_event__bindgen_ty_1__bindgen_ty_8 {
            ip: 11,
            mode: pt_exec_mode_ptem_32bit,
        };

        let payload: Payload = evt.into();
        match payload {
            Payload::ExecMode(e) => {
                assert_eq!(e.ip(), 11);
                assert_eq!(e.mode(), ExecModeType::Bit32);
            }
            _ => unreachable!("oof"),
        }
    }
}
