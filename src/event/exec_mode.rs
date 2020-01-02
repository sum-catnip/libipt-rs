use std::convert::TryFrom;
use libipt_sys::{
    pt_event__bindgen_ty_1__bindgen_ty_8,
    pt_exec_mode_ptem_16bit,
    pt_exec_mode_ptem_32bit,
    pt_exec_mode_ptem_64bit,
    pt_exec_mode_ptem_unknown
};

use num_enum::TryFromPrimitive;

#[derive(Clone, Copy, TryFromPrimitive)]
#[repr(i32)]
pub enum ExecModeType {
    Bit16 = pt_exec_mode_ptem_16bit,
    Bit32 = pt_exec_mode_ptem_32bit,
    Bit64 = pt_exec_mode_ptem_64bit,
    Unknown = pt_exec_mode_ptem_unknown
}

/// An execution mode change
#[derive(Clone, Copy)]
pub struct ExecMode(pub(super) pt_event__bindgen_ty_1__bindgen_ty_8);
impl ExecMode {
    /// The address at which the event is effective
    pub fn ip(self) -> u64 { self.0.ip }
    /// The execution mode
    pub fn mode(self) -> ExecModeType { ExecModeType::try_from(self.0.mode).unwrap() }
}