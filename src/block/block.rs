use crate::insn::Class;
use crate::event::ExecModeType;
use std::convert::TryFrom;
use libipt_sys::pt_block;

#[cfg(test)]
mod test {
    use super::*;
    use libipt_sys::{
        pt_exec_mode_ptem_32bit,
        pt_insn_class_ptic_error,
    };

    #[test]
    fn test_block_props() {
        let data: [u8; 15] = [17; 15];
        let blk = Block(pt_block {
            ip: 1,
            end_ip: 2,
            isid: 3,
            mode: pt_exec_mode_ptem_32bit,
            iclass: pt_insn_class_ptic_error,
            ninsn: 4,
            raw: data,
            size: 8,
            _bitfield_1: pt_block::new_bitfield_1(0, 1),
            __bindgen_padding_0: Default::default()
       });

       assert_eq!(blk.ip(), 1);
       assert_eq!(blk.end_ip(), 2);
       assert_eq!(blk.isid(), 3);
       assert_eq!(blk.mode(), ExecModeType::Bit32);
       assert_eq!(blk.class(), Class::Error);
       assert_eq!(blk.ninsn(), 4);
       assert_eq!(blk.raw(), &data[..8]);
       assert!(blk.truncated());
       assert!(!blk.speculative());
    }

    #[test]
    fn test_block_notruncate() {
        let data: [u8; 15] = [17; 15];
        let blk = Block(pt_block {
            ip: 1,
            end_ip: 2,
            isid: 3,
            mode: pt_exec_mode_ptem_32bit,
            iclass: pt_insn_class_ptic_error,
            ninsn: 4,
            raw: data,
            size: 8,
            _bitfield_1: pt_block::new_bitfield_1(0, 0),
            __bindgen_padding_0: Default::default()
       });

       assert_eq!(blk.ip(), 1);
       assert_eq!(blk.end_ip(), 2);
       assert_eq!(blk.isid(), 3);
       assert_eq!(blk.mode(), ExecModeType::Bit32);
       assert_eq!(blk.class(), Class::Error);
       assert_eq!(blk.ninsn(), 4);
       assert!(blk.raw().len() > 0);
       assert!(!blk.truncated());
       assert!(!blk.speculative());
    }
}

/// A block of instructions.
///
/// Instructions in this block are executed sequentially but are not necessarily
/// contiguous in memory.  Users are expected to follow direct branches.
#[derive(Clone, Copy)]
pub struct Block(pub(super) pt_block);
impl Block {
    /// The IP of the first instruction in this block.
    pub fn ip(&self) -> u64 { self.0.ip }

    /// The IP of the last instruction in this block.
    ///
    /// This can be used for error-detection.
    pub fn end_ip(&self) -> u64 { self.0.end_ip }

    /// The image section that contains the instructions in this block.
    ///
    /// A value of zero means that the section did not have an identifier.
    /// The section was not added via an image section cache or the memory
    /// was read via the read memory callback.
    pub fn isid(&self) -> i32 { self.0.isid }

    /// The execution mode for all instructions in this block.
    pub fn mode(&self) -> ExecModeType {
        ExecModeType::try_from(self.0.mode).unwrap()
    }

    /// The instruction class for the last instruction in this block.
    ///
    /// This field may be set to Class::Error to indicate that the instruction
    /// class is not available. The block decoder may choose to not provide
    /// the instruction class in some cases for performance reasons.
    pub fn class(&self) -> Class {
        Class::try_from(self.0.iclass).unwrap()
    }

    /// The number of instructions in this block.
    pub fn ninsn(&self) -> u16 { self.0.ninsn }

    /// The raw bytes of the last instruction in this block in case the
    /// instruction does not fit entirely into this block's section.
    ///
    /// This field is only valid if truncated is set.
    pub fn raw(&self) -> &[u8] {
        &self.0.raw[..self.0.size as usize]
    }

    /// A collection of flags giving additional information about the
    /// instructions in this block.
    ///
    /// - all instructions in this block were executed speculatively.
    pub fn speculative(&self) -> bool { self.0.speculative() > 0 }

    /// A collection of flags giving additional information about the
    /// instructions in this block.
    ///
    /// - the last instruction in this block is truncated.
    ///
    /// It starts in this block's section but continues in one or more
    /// other sections depending on how fragmented the memory image is.
    ///
    /// The raw bytes for the last instruction are provided in \@raw and
    /// its size in \@size in this case.
    pub fn truncated(&self) -> bool { self.0.truncated() > 0 }
}
