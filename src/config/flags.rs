use libipt_sys::{
    pt_conf_flags,
    pt_conf_flags__bindgen_ty_1,
    pt_conf_flags__bindgen_ty_1__bindgen_ty_1,
    pt_conf_flags__bindgen_ty_1__bindgen_ty_2,
    pt_conf_flags__bindgen_ty_1__bindgen_ty_3,
    __BindgenBitfieldUnit
};

use bitflags::bitflags;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_block_flags() {
        let blk: BlockFlags = BlockFlags::END_ON_CALL | BlockFlags::END_ON_JUMP;
        let raw: pt_conf_flags = blk.into();

        unsafe {
            assert_eq!(raw.variant.block.end_on_call(), 1);
            assert_eq!(raw.variant.block.enable_tick_events(), 0);
            assert_eq!(raw.variant.block.end_on_jump(), 1);
            assert_eq!(raw.variant.block.keep_tcal_on_ovf(), 0);
        }
        
        let blk: BlockFlags = BlockFlags::END_ON_CALL |
        BlockFlags::END_ON_JUMP |
        BlockFlags::ENABLE_TICK_EVENTS |
        BlockFlags::KEEP_TCAL_ON_OVF;
        let raw: pt_conf_flags = blk.into();

        unsafe {
            assert_eq!(raw.variant.block.end_on_call(), 1);
            assert_eq!(raw.variant.block.enable_tick_events(), 1);
            assert_eq!(raw.variant.block.end_on_jump(), 1);
            assert_eq!(raw.variant.block.keep_tcal_on_ovf(), 1);
        }
    }

    #[test]
    fn test_insn_flags() {
        let insn = InsnFlags::ENABLE_TICK_EVENTS;
        let raw: pt_conf_flags = insn.into();

        unsafe {
            assert_eq!(raw.variant.insn.enable_tick_events(), 1);
            assert_eq!(raw.variant.insn.keep_tcal_on_ovf(), 0);
        }

        let insn = InsnFlags::ENABLE_TICK_EVENTS | InsnFlags::KEEP_TCAL_ON_OVF;
        let raw: pt_conf_flags = insn.into();

        unsafe {
            assert_eq!(raw.variant.insn.enable_tick_events(), 1);
            assert_eq!(raw.variant.insn.keep_tcal_on_ovf(), 1);
        }
    }

    #[test]
    fn test_query_flags() {
        let query = QueryFlags::empty();
        let raw: pt_conf_flags = query.into();

        unsafe { assert_eq!(raw.variant.query.keep_tcal_on_ovf(), 0); }

        let query: QueryFlags = QueryFlags::KEEP_TCAL_ON_OVF;
        let raw: pt_conf_flags = query.into();

        unsafe { assert_eq!(raw.variant.query.keep_tcal_on_ovf(), 1); }
    }
}

bitflags! {
    /// flags for the block decoder
    pub struct BlockFlags: u8 {
        /// End a block after a call instruction
        const END_ON_CALL        = 0b00000001;
        /// Enable tick events for timing updates
        const ENABLE_TICK_EVENTS = 0b00000010;
        /// End a block after a jump instruction
        const END_ON_JUMP        = 0b00000100;
        /// Preserve timing calibration on overflow
        const KEEP_TCAL_ON_OVF   = 0b00001000;
    }
}

bitflags! {
    /// flags for the instruction flow decoder
    pub struct InsnFlags: u8 {
        /// Enable tick events for timing updates
        const ENABLE_TICK_EVENTS = 0b00000001;
        /// Preserve timing calibration on overflow
        const KEEP_TCAL_ON_OVF   = 0b00000010;
    }
}

bitflags! {
    /// flags for the query decoder
    pub struct QueryFlags: u8 {
        /// Preserve timing calibration on overflow
        const KEEP_TCAL_ON_OVF = 0b00000001;
    }
}

impl From<BlockFlags> for pt_conf_flags {
    fn from(flags: BlockFlags) -> Self {
        pt_conf_flags {
            variant: pt_conf_flags__bindgen_ty_1 {
                block: pt_conf_flags__bindgen_ty_1__bindgen_ty_1 {
                    _bitfield_1: __BindgenBitfieldUnit::new([flags.bits()]),
                    __bindgen_padding_0: Default::default() }}}
    }
}

impl From<InsnFlags> for pt_conf_flags {
    fn from(flags: InsnFlags) -> Self {
        pt_conf_flags {
            variant: pt_conf_flags__bindgen_ty_1 {
                insn: pt_conf_flags__bindgen_ty_1__bindgen_ty_2 {
                    _bitfield_1: __BindgenBitfieldUnit::new([flags.bits()]),
                    __bindgen_padding_0: Default::default() }}}
    }
}

impl From<QueryFlags> for pt_conf_flags {
    fn from(flags: QueryFlags) -> Self {
        pt_conf_flags {
            variant: pt_conf_flags__bindgen_ty_1 {
                query: pt_conf_flags__bindgen_ty_1__bindgen_ty_3 {
                    _bitfield_1: __BindgenBitfieldUnit::new([flags.bits()]),
                    __bindgen_padding_0: Default::default() }}}
    }
}