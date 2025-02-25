use libipt_sys::{pt_packet, pt_packet_type_ppt_stop};
use std::mem;

#[derive(Clone, Copy, Debug)]
pub struct Stop {}

impl Default for Stop {
    fn default() -> Self {
        Self::new()
    }
}

impl Stop {
    pub fn new() -> Self {
        Stop {}
    }
}

impl From<Stop> for pt_packet {
    fn from(_: Stop) -> Self {
        pt_packet {
            type_: pt_packet_type_ppt_stop,
            size: mem::size_of::<pt_packet>() as u8,
            payload: unsafe { mem::zeroed() },
        }
    }
}

impl From<pt_packet> for Stop {
    fn from(_val: pt_packet) -> Self {
        Stop {}
    }
}
