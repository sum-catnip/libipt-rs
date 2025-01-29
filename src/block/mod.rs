// Certain casts are required only on Windows. Inform Clippy to ignore them.
#![allow(clippy::unnecessary_cast)]

use crate::event::ExecModeType;
use crate::insn::Class;
use libipt_sys::pt_block;
use std::convert::TryFrom;

mod decoder;
pub use decoder::*;

/// A block of instructions.
///
/// Instructions in this block are executed sequentially but are not necessarily
/// contiguous in memory. Users are expected to follow direct branches.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Block(pub(super) pt_block);
impl Block {
    /// The IP of the first instruction in this block.
    #[must_use]
    pub fn ip(&self) -> u64 {
        self.0.ip
    }

    /// The IP of the last instruction in this block.
    ///
    /// This can be used for error-detection.
    #[must_use]
    pub fn end_ip(&self) -> u64 {
        self.0.end_ip
    }

    // TODO: make this more rusty? at least with an Option? &Image?
    /// The image section that contains the instructions in this block.
    ///
    /// A value of zero means that the section did not have an identifier.
    /// The section was not added via an image section cache or the memory
    /// was read via the read memory callback.
    #[must_use]
    pub fn isid(&self) -> i32 {
        self.0.isid
    }

    /// The execution mode for all instructions in this block.
    #[must_use]
    #[expect(clippy::missing_panics_doc)]
    pub fn mode(&self) -> ExecModeType {
        ExecModeType::try_from(self.0.mode as u32).unwrap()
    }

    /// The instruction class for the last instruction in this block.
    ///
    /// This field may be set to `Class::Unknown` to indicate that the instruction
    /// class is not available. The block decoder may choose to not provide
    /// the instruction class in some cases for performance reasons.
    #[must_use]
    #[expect(clippy::missing_panics_doc)]
    pub fn class(&self) -> Class {
        Class::try_from(self.0.iclass as u32).unwrap()
    }

    /// The number of instructions in this block.
    #[must_use]
    pub fn ninsn(&self) -> u16 {
        self.0.ninsn
    }

    /// The raw bytes of the last instruction in this block in case the
    /// instruction does not fit entirely into this block's section.
    ///
    /// This field is `Some`only if `truncated()` is true.
    #[must_use]
    pub fn raw(&self) -> Option<&[u8]> {
        if self.truncated() {
            Some(&self.0.raw[..self.0.size as usize])
        } else {
            None
        }
    }

    /// All instructions in this block were executed speculatively.
    #[must_use]
    pub fn speculative(&self) -> bool {
        self.0.speculative() > 0
    }

    /// The last instruction in this block is truncated.
    ///
    /// It starts in this block's section but continues in one or more
    /// other sections depending on how fragmented the memory image is.
    ///
    /// The raw bytes for the last instruction are provided in @raw and
    /// its size in @size in this case.
    #[must_use]
    pub fn truncated(&self) -> bool {
        self.0.truncated() > 0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use libipt_sys::{pt_exec_mode_ptem_32bit, pt_insn_class_ptic_error};

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
            _bitfield_align_1: [],
            _bitfield_1: pt_block::new_bitfield_1(0, 1),
            __bindgen_padding_0: Default::default(),
        });

        assert_eq!(blk.ip(), 1);
        assert_eq!(blk.end_ip(), 2);
        assert_eq!(blk.isid(), 3);
        assert_eq!(blk.mode(), ExecModeType::Bit32);
        assert_eq!(blk.class(), Class::Unknown);
        assert_eq!(blk.ninsn(), 4);
        assert_eq!(blk.raw(), Some(&data[..8]));
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
            _bitfield_align_1: [],
            _bitfield_1: pt_block::new_bitfield_1(0, 0),
            __bindgen_padding_0: Default::default(),
        });

        assert_eq!(blk.ip(), 1);
        assert_eq!(blk.end_ip(), 2);
        assert_eq!(blk.isid(), 3);
        assert_eq!(blk.mode(), ExecModeType::Bit32);
        assert_eq!(blk.class(), Class::Unknown);
        assert_eq!(blk.ninsn(), 4);
        assert!(blk.raw().is_none());
        assert!(!blk.truncated());
        assert!(!blk.speculative());
    }
}
