use crate::event::ExecModeType;
use super::Class;

use std::convert::TryFrom;

use libipt_sys::pt_insn;

/// A single traced instruction.
#[derive(Clone, Copy)]
pub struct Insn(pub(crate) pt_insn);
impl Insn {
    /// The virtual address in its process.
    pub fn ip(self) -> u64 { self.0.ip }

    /// The image section identifier for the section containing this instruction.
    ///
    /// None if the section was not added via an image section cache or the memory was read via the read memory callback.
    pub fn isid(self) -> Option<i32> {
        match self.0.isid { 0 => None, x => Some(x) }
    }

    /// The execution mode.
    pub fn exec_mode(self) -> ExecModeType {
        ExecModeType::try_from(self.0.mode)
            .expect(concat!("unmatched ExecModeType enum value, ",
                "this is a bug in either libipt or the bindings"))
    }

    /// A coarse classification.
    pub fn class(self) -> Class {
        Class::try_from(self.0.iclass)
            .expect(concat!("unmatched Class enum value, ",
                "this is a bug in either libipt or the bindings"))
    }

    /// The size in bytes.
    pub fn raw(&self) -> &[u8] {
        &self.0.raw[..self.0.size as usize]
    }

    /// A collection of flags giving additional information:
    ///
    /// - the instruction was executed speculatively.
    pub fn speculative(self) -> bool { self.0.speculative() > 0 }
    
    /// A collection of flags giving additional information:
    ///
    /// - this instruction is truncated in its image section.
    ///
    /// It starts in the image section identified by \@isid and continues
    /// in one or more other sections.
    pub fn truncated(self) -> bool { self.0.truncated() > 0 }
}