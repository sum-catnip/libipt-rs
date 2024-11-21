use std::mem;
use libipt_sys::{pt_packet, pt_packet_type_ppt_psbend};

#[derive(Clone, Copy, Debug)]
pub struct Psbend {}

impl Default for Psbend {
    fn default() -> Self {
        Self::new()
    }
}

impl Psbend {
    pub fn new() -> Self { Psbend {} }
}

impl From<Psbend> for pt_packet {
    fn from(_: Psbend) -> Self {
        pt_packet {
            type_: pt_packet_type_ppt_psbend,
            size: mem::size_of::<pt_packet>() as u8,
            payload: unsafe { mem::zeroed() }
        }
    }
}

impl From<pt_packet> for Psbend {
    fn from(_val: pt_packet) -> Self { Psbend{} }
}
