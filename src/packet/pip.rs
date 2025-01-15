use libipt_sys::{__BindgenBitfieldUnit, pt_packet_pip, pt_packet_type_ppt_pip};

/// A PIP packet.
/// Packet: pip
#[derive(Clone, Copy, Debug)]
pub struct Pip(pt_packet_pip);
impl Pip {
    #[inline]
    pub fn new(cr3: u64, nr: bool) -> Self {
        Pip(pt_packet_pip {
            cr3,
            _bitfield_align_1: [],
            _bitfield_1: __BindgenBitfieldUnit::new([nr as u8]),
            __bindgen_padding_0: Default::default(),
        })
    }

    /// The CR3 value
    #[inline]
    #[must_use]
    pub fn cr3(self) -> u64 {
        self.0.cr3
    }
    #[inline]
    /// The CR3 value
    pub fn set_cr3(&mut self, cr3: u64) {
        self.0.cr3 = cr3;
    }

    /// The non-root bit
    #[inline]
    #[must_use]
    pub fn nr(&self) -> bool {
        self.0.nr() > 0
    }
    #[inline]
    /// The non-root bit
    pub fn set_nr(&mut self, nr: bool) {
        self.0.set_nr(nr as u32)
    }
}

wrap2raw!(Pip, pt_packet_type_ppt_pip, pip);
raw2wrap!(Pip, Pip, pt_packet_pip);
