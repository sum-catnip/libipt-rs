use libipt_sys::{pt_packet, pt_packet_type_ppt_ovf};
use std::mem;

#[derive(Clone, Copy, Debug)]
pub struct Ovf {}

impl Default for Ovf {
    fn default() -> Self {
        Self::new()
    }
}

impl Ovf {
    pub fn new() -> Self {
        Ovf {}
    }
}

impl From<Ovf> for pt_packet {
    fn from(_: Ovf) -> Self {
        pt_packet {
            type_: pt_packet_type_ppt_ovf,
            size: mem::size_of::<pt_packet>() as u8,
            payload: unsafe { mem::zeroed() },
        }
    }
}

impl From<pt_packet> for Ovf {
    fn from(_val: pt_packet) -> Self {
        Ovf {}
    }
}
