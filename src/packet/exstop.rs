use libipt_sys::{__BindgenBitfieldUnit, pt_packet_exstop, pt_packet_type_ppt_exstop};

/// A EXSTOP packet.
/// Packet: exstop
#[derive(Clone, Copy, Debug)]
pub struct Exstop(pt_packet_exstop);
impl Exstop {
    #[inline]
    #[must_use]
    pub fn new(ip: bool) -> Self {
        Exstop(pt_packet_exstop {
            _bitfield_align_1: [],
            _bitfield_1: __BindgenBitfieldUnit::new([ip as u8]),
            __bindgen_padding_0: Default::default(),
        })
    }

    /// A flag specifying the binding of the packet:
    ///
    /// set:   binds to the next FUP.
    /// clear: standalone
    #[inline]
    #[must_use]
    pub fn ip(&self) -> bool {
        self.0.ip() > 0
    }

    /// A flag specifying the binding of the packet:
    ///
    /// set:   binds to the next FUP.
    /// clear: standalone
    #[inline]
    pub fn set_ip(&mut self, ip: bool) {
        self.0.set_ip(ip as u32);
    }
}

wrap2raw!(Exstop, pt_packet_type_ppt_exstop, exstop);
raw2wrap!(Exstop, Exstop, pt_packet_exstop);
