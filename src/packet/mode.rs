use bitflags::bitflags;
use libipt_sys::{
    pt_mode_leaf_pt_mol_exec as PT_MODE_LEAF_PT_MOL_EXEC,
    pt_mode_leaf_pt_mol_tsx  as PT_MODE_LEAF_PT_MOL_TSX,
    pt_packet_mode,
    pt_packet_type_ppt_mode,
    pt_packet_mode_exec,
    pt_packet_mode_tsx,
    __BindgenBitfieldUnit,
    pt_packet_mode__bindgen_ty_1
};

bitflags! {
    /// A mode.exec packet
    pub struct Exec : u32 {
        /// The mode.exec csl bit
        const CSL = 0b00000001;
        /// The mode.exec csd bit
        const CSD = 0b00000010;
    }
}

bitflags! {
    /// A mode.tsx packet
    pub struct Tsx : u32 {
        /// The mode.tsx intx bit
        const INTX = 0b00000001;
        /// The mode.tsx abrt bit
        const ABRT = 0b00000010;
    }
}

impl Into<pt_packet_mode__bindgen_ty_1> for Exec {
    fn into(self) -> pt_packet_mode__bindgen_ty_1 {
        pt_packet_mode__bindgen_ty_1 {
            exec: pt_packet_mode_exec {
                _bitfield_1: __BindgenBitfieldUnit::new([self.bits() as u8]),
                __bindgen_padding_0: Default::default()
            }
        }
    }
}

impl Into<pt_packet_mode__bindgen_ty_1> for Tsx {
    fn into(self) -> pt_packet_mode__bindgen_ty_1 {
        pt_packet_mode__bindgen_ty_1 {
            tsx: pt_packet_mode_tsx {
                _bitfield_1: __BindgenBitfieldUnit::new([self.bits() as u8]),
                __bindgen_padding_0: Default::default()
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum Payload {
    /// A mode.exec packet.
    Exec(Exec),
    /// A mode.tsx packet.
    Tsx(Tsx)
}

/// A mode packet.
/// Packet: mode
#[derive(Clone, Copy)]
pub struct Mode (pt_packet_mode);
impl Mode {
    #[inline]
    pub fn new(payload: Payload) -> Self {
        // i know this looks a bit wonky but its the fastest way
        // to convert the bits enum into the union
        Mode(match payload {
            Payload::Exec(e) => pt_packet_mode {
                leaf: PT_MODE_LEAF_PT_MOL_EXEC, bits: e.into()
            },
            Payload::Tsx(t) => pt_packet_mode {
                leaf: PT_MODE_LEAF_PT_MOL_TSX, bits: t.into()
            }
        })
    }

    /// Gets the payload of this packet as an enum.
    /// Intel calls this field `bits`
   #[inline]
    pub fn payload(self) -> Payload {
        match self.0.leaf {
            PT_MODE_LEAF_PT_MOL_EXEC => Payload::Exec(
                Exec::from_bits(unsafe {
                    self.0.bits.exec._bitfield_1.get(0, 2)
                } as u32).unwrap()
            ),

            PT_MODE_LEAF_PT_MOL_TSX => Payload::Tsx(
                Tsx::from_bits(unsafe {
                    self.0.bits.tsx._bitfield_1.get(0, 2)
                } as u32).unwrap()
            ),

            _ => unreachable!()
        }
    }

    #[inline]
    pub fn set_payload(&mut self, payload: Payload) {
        match payload {
            Payload::Exec(e) => self.0.bits = e.into(),
            Payload::Tsx(t)  => self.0.bits = t.into()
        }
    }
} 

wrap2raw!(Mode, pt_packet_type_ppt_mode, mode);
raw2wrap!(Mode, Mode, pt_packet_mode);