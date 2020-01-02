use num_enum::TryFromPrimitive;
use libipt_sys::{
    pt_insn_class_ptic_call,
    pt_insn_class_ptic_cond_jump,
    pt_insn_class_ptic_error,
    pt_insn_class_ptic_far_call,
    pt_insn_class_ptic_far_jump,
    pt_insn_class_ptic_far_return,
    pt_insn_class_ptic_jump,
    pt_insn_class_ptic_other,
    pt_insn_class_ptic_ptwrite,
    pt_insn_class_ptic_return
};

#[derive(Clone, Copy, TryFromPrimitive)]
#[repr(i32)]
pub enum Class {
    /// The instruction is a near (function) call.
    Call = pt_insn_class_ptic_call,
    /// The instruction is a near conditional jump.
    CondJump = pt_insn_class_ptic_cond_jump,
    /// The instruction could not be classified.
    Error = pt_insn_class_ptic_error,
    /// The instruction is a call-like far transfer.
    /// E.g. SYSCALL, SYSENTER, or FAR CALL.
    FarCall = pt_insn_class_ptic_far_call,
    /// The instruction is a jump-like far transfer.
    /// E.g. FAR JMP.
    FarJump = pt_insn_class_ptic_far_jump,
    /// The instruction is a return-like far transfer.
    /// E.g. SYSRET, SYSEXIT, IRET, or FAR RET.
    FarReturn = pt_insn_class_ptic_far_return,
    /// The instruction is a near unconditional jump.
    Jump = pt_insn_class_ptic_jump,
    /// The instruction is something not listed below.
    Other = pt_insn_class_ptic_other,
    /// The instruction is a PTWRITE.
    Ptwrite = pt_insn_class_ptic_ptwrite,
    /// The instruction is a near (function) return.
    Return = pt_insn_class_ptic_return
}