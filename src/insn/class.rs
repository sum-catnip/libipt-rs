// Certain casts are required only on Windows. Inform Clippy to ignore them.
#![allow(clippy::unnecessary_cast)]

use libipt_sys::{
    pt_insn_class_ptic_call, pt_insn_class_ptic_cond_jump, pt_insn_class_ptic_far_call,
    pt_insn_class_ptic_far_jump, pt_insn_class_ptic_far_return, pt_insn_class_ptic_jump,
    pt_insn_class_ptic_other, pt_insn_class_ptic_ptwrite, pt_insn_class_ptic_return,
    pt_insn_class_ptic_unknown,
};
use num_enum::TryFromPrimitive;

/// The instruction class.
///
/// We provide only a very coarse classification suitable for reconstructing
/// the execution flow.
#[derive(Clone, Copy, Debug, TryFromPrimitive, PartialEq)]
#[repr(u32)]
pub enum Class {
    /// The instruction is a near (function) call.
    Call = pt_insn_class_ptic_call as u32,
    /// The instruction is a near conditional jump.
    CondJump = pt_insn_class_ptic_cond_jump as u32,
    /// The instruction could not be classified.
    Unknown = pt_insn_class_ptic_unknown as u32,
    /// The instruction is a call-like far transfer.
    /// E.g. SYSCALL, SYSENTER, or FAR CALL.
    FarCall = pt_insn_class_ptic_far_call as u32,
    /// The instruction is a jump-like far transfer.
    /// E.g. FAR JMP.
    FarJump = pt_insn_class_ptic_far_jump as u32,
    /// The instruction is a return-like far transfer.
    /// E.g. SYSRET, SYSEXIT, IRET, or FAR RET.
    FarReturn = pt_insn_class_ptic_far_return as u32,
    /// The instruction is a near unconditional jump.
    Jump = pt_insn_class_ptic_jump as u32,
    /// The instruction is something not listed below.
    Other = pt_insn_class_ptic_other as u32,
    /// The instruction is a PTWRITE.
    Ptwrite = pt_insn_class_ptic_ptwrite as u32,
    /// The instruction is a near (function) return.
    Return = pt_insn_class_ptic_return as u32,
}
