use std::mem;
use libipt_sys::{pt_packet, pt_packet_type_ppt_pad};

#[derive(Clone, Copy)]
pub struct Pad {}

impl Pad {
    pub fn new() -> Self { Pad {} }
}

impl From<Pad> for pt_packet {
    fn from(_: Pad) -> Self {
        pt_packet {
            type_: pt_packet_type_ppt_pad,
            size: mem::size_of::<pt_packet>() as u8,
            payload: unsafe { mem::zeroed() }
        }
    }
}

impl Into<Pad> for pt_packet {
    fn into(self) -> Pad { Pad{} }
}