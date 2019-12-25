use libipt_sys::{
    pt_conf_flags,
    pt_conf_flags__bindgen_ty_1,
    pt_conf_flags__bindgen_ty_1__bindgen_ty_1,
    pt_conf_flags__bindgen_ty_1__bindgen_ty_2,
    pt_conf_flags__bindgen_ty_1__bindgen_ty_3,
    __BindgenBitfieldUnit
};

use bitflags::bitflags;

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

/// a collection of decoder-specific configuration flags
pub enum DecoderFlags {
    /// flags for the block decoder
    Block(BlockFlags),
    /// flags for the instruction flow decoder
    Insn(InsnFlags),
    /// flags for the query decoder
    Query(QueryFlags)
}

impl From<DecoderFlags> for pt_conf_flags {
    fn from(flags: DecoderFlags) -> Self {
        // yeah i know.. this looks absolutely disgusting
        // i just wanted a decent rustic abstraction for the union fields
        match flags {
            DecoderFlags::Block(f) => pt_conf_flags {
                variant: pt_conf_flags__bindgen_ty_1 {
                    block: pt_conf_flags__bindgen_ty_1__bindgen_ty_1 {
                        _bitfield_1: __BindgenBitfieldUnit::new([f.bits()]),
                        __bindgen_padding_0: Default::default() } } },

            DecoderFlags::Insn(f) => pt_conf_flags {
                variant: pt_conf_flags__bindgen_ty_1 {
                    insn: pt_conf_flags__bindgen_ty_1__bindgen_ty_2 {
                        _bitfield_1: __BindgenBitfieldUnit::new([f.bits()]),
                        __bindgen_padding_0: Default::default() } } },

            DecoderFlags::Query(f) => pt_conf_flags {
                variant: pt_conf_flags__bindgen_ty_1 {
                    query: pt_conf_flags__bindgen_ty_1__bindgen_ty_3 {
                        _bitfield_1: __BindgenBitfieldUnit::new([f.bits()]),
                        __bindgen_padding_0: Default::default() } } },
       }
    }
}

impl Default for DecoderFlags {
    fn default() -> Self { DecoderFlags::Block(BlockFlags::empty()) }
}