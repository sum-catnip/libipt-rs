use std::convert::TryFrom;
use libipt_sys::{
    pt_event__bindgen_ty_1__bindgen_ty_8,
    pt_exec_mode_ptem_16bit,
    pt_exec_mode_ptem_32bit,
    pt_exec_mode_ptem_64bit,
    pt_exec_mode_ptem_unknown
};

use num_enum::TryFromPrimitive;

#[cfg(test)]
mod test {
    use super::*;
    use super::super::Payload;
    use std::mem;
    use libipt_sys::{ pt_event, pt_event_type_ptev_exec_mode };

    #[test]
    fn test_exec_mode_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_exec_mode;
        evt.variant.exec_mode = pt_event__bindgen_ty_1__bindgen_ty_8 {
            ip: 11,
            mode: pt_exec_mode_ptem_32bit
        };

        let payload: Payload = evt.into();
        match payload {
            Payload::ExecMode(e) => {
                assert_eq!(e.ip(), 11);
                assert_eq!(e.mode(), ExecModeType::Bit32);
            },
            _ => unreachable!("oof")
        }
    }
}

#[derive(Clone, Copy, TryFromPrimitive, Debug, PartialEq)]
#[repr(i32)]
pub enum ExecModeType {
    Bit16 = pt_exec_mode_ptem_16bit,
    Bit32 = pt_exec_mode_ptem_32bit,
    Bit64 = pt_exec_mode_ptem_64bit,
    Unknown = pt_exec_mode_ptem_unknown
}

/// An execution mode change
#[derive(Clone, Copy, Debug)]
pub struct ExecMode(pub(super) pt_event__bindgen_ty_1__bindgen_ty_8);
impl ExecMode {
    /// The address at which the event is effective
    pub fn ip(self) -> u64 { self.0.ip }
    /// The execution mode
    pub fn mode(self) -> ExecModeType { ExecModeType::try_from(self.0.mode).unwrap() }
}