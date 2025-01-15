use libipt_sys::{__BindgenBitfieldUnit, pt_packet_ptw, pt_packet_type_ppt_ptw};

/// A PTW packet.
/// Packet: ptw
#[derive(Clone, Copy, Debug)]
pub struct Ptw(pt_packet_ptw);
impl Ptw {
    #[inline]
    #[must_use]
    pub fn new(payload: u64, plc: u8, ip: bool) -> Self {
        Ptw(pt_packet_ptw {
            payload,
            plc,
            _bitfield_align_1: [],
            _bitfield_1: __BindgenBitfieldUnit::new([ip as u8]),
            __bindgen_padding_0: Default::default(),
        })
    }

    /// The raw payload
    #[inline]
    #[must_use]
    pub fn payload(&self) -> u64 {
        self.0.payload
    }

    /// The raw payload
    #[inline]
    pub fn set_payload(&mut self, payload: u64) {
        self.0.payload = payload;
    }

    /// The payload size as encoded in the packet
    #[inline]
    #[must_use]
    pub fn plc(&self) -> u8 {
        self.0.plc
    }

    /// The payload size as encoded in the packet
    #[inline]
    pub fn set_plc(&mut self, plc: u8) {
        self.0.plc = plc;
    }

    /// A flag saying whether a FUP is following PTW that provides
    /// the IP of the corresponding PTWRITE instruction.
    #[inline]
    #[must_use]
    pub fn ip(&self) -> bool {
        self.0.ip() > 0
    }

    /// A flag saying whether a FUP is following PTW that provides
    /// the IP of the corresponding PTWRITE instruction.
    #[inline]
    pub fn set_ip(&mut self, ip: bool) {
        self.0.set_ip(ip as u32);
    }
}

wrap2raw!(Ptw, pt_packet_type_ppt_ptw, ptw);
raw2wrap!(Ptw, Ptw, pt_packet_ptw);
