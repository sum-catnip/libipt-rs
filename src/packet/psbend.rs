use std::mem;
use libipt_sys::{pt_packet, pt_packet_type_ppt_psbend};

#[derive(Clone, Copy)]
pub struct Psbend {}

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

impl Into<Psbend> for pt_packet {
    fn into(self) -> Psbend { Psbend{} }
}