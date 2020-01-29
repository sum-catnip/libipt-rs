use std::mem;
use libipt_sys::{pt_packet, pt_packet_type_ppt_stop};

#[derive(Clone, Copy)]
pub struct Stop {}

impl Stop {
    pub fn new() -> Self { Stop {} }
}

impl From<Stop> for pt_packet {
    fn from(_: Stop) -> Self {
        pt_packet {
            type_: pt_packet_type_ppt_stop,
            size: mem::size_of::<pt_packet>() as u8,
            payload: unsafe { mem::zeroed() }
        }
    }
}

impl Into<Stop> for pt_packet {
    fn into(self) -> Stop { Stop{} }
}