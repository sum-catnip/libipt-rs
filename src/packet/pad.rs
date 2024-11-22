use libipt_sys::{pt_packet, pt_packet_type_ppt_pad};
use std::mem;

#[derive(Clone, Copy, Debug)]
pub struct Pad {}

impl Default for Pad {
    fn default() -> Self {
        Self::new()
    }
}

impl Pad {
    pub fn new() -> Self {
        Pad {}
    }
}

impl From<Pad> for pt_packet {
    fn from(_: Pad) -> Self {
        pt_packet {
            type_: pt_packet_type_ppt_pad,
            size: mem::size_of::<pt_packet>() as u8,
            payload: unsafe { mem::zeroed() },
        }
    }
}

impl From<pt_packet> for Pad {
    fn from(_val: pt_packet) -> Self {
        Pad {}
    }
}
