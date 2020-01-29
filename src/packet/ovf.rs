use std::mem;
use libipt_sys::{pt_packet, pt_packet_type_ppt_ovf};

#[derive(Clone, Copy)]
pub struct Ovf {}

impl Ovf {
    pub fn new() -> Self { Ovf {} }
}

impl From<Ovf> for pt_packet {
    fn from(_: Ovf) -> Self {
        pt_packet {
            type_: pt_packet_type_ppt_ovf,
            size: mem::size_of::<pt_packet>() as u8,
            payload: unsafe { mem::zeroed() }
        }
    }
}

impl Into<Ovf> for pt_packet {
    fn into(self) -> Ovf{ Ovf{} }
}