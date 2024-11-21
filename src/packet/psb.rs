use std::mem;
use libipt_sys::{pt_packet, pt_packet_type_ppt_psb};

#[derive(Clone, Copy, Debug)]
pub struct Psb {}

impl Default for Psb {
    fn default() -> Self {
        Self::new()
    }
}

impl Psb {
    pub fn new() -> Self { Psb {} }
}

impl From<Psb> for pt_packet {
    fn from(_: Psb) -> Self {
        pt_packet {
            type_: pt_packet_type_ppt_psb,
            size: mem::size_of::<pt_packet>() as u8,
            payload: unsafe { mem::zeroed() }
        }
    }
}

impl From<pt_packet> for Psb {
    fn from(_val: pt_packet) -> Self { Psb{} }
}
