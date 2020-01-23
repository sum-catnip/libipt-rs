use crate::event::ExecModeType;
use super::Class;

use std::convert::TryFrom;

use libipt_sys::pt_insn;

#[cfg(test)]
mod test {
    use super::*;
    use libipt_sys::pt_insn_class_ptic_call;
    use libipt_sys::pt_exec_mode_ptem_32bit;

    #[test]
    fn test_insn_props() {
        let data: [u8; 15] = [17; 15];
        let blk = Insn(pt_insn{
            ip: 1,
            isid: 2,
            mode: pt_exec_mode_ptem_32bit,
            iclass: pt_insn_class_ptic_call,
            raw: data,
            size: 8,
            _bitfield_1: pt_insn::new_bitfield_1(0, 1),
            __bindgen_padding_0: Default::default()
       });

       assert_eq!(blk.ip(), 1);
       assert_eq!(blk.isid(), 2);
       assert_eq!(blk.mode(), ExecModeType::Bit32);
       assert_eq!(blk.class(), Class::Call);
       assert_eq!(blk.raw(), &data[..8]);
       assert!(blk.truncated());
       assert!(!blk.speculative());
    }
}

/// A single traced instruction.
#[derive(Clone, Copy)]
pub struct Insn(pub(crate) pt_insn);
impl Insn {
    /// The virtual address in its process.
    pub fn ip(self) -> u64 { self.0.ip }

    /// The image section identifier for the section containing this instruction.
    ///
    /// A value of zero means that the section did not have an identifier.
    pub fn isid(self) -> i32 { self.0.isid }

    /// The execution mode.
    pub fn mode(self) -> ExecModeType {
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