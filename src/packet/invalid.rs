use std::mem;
use libipt_sys::{pt_packet, pt_packet_type_ppt_invalid};

#[derive(Clone, Copy)]
pub struct Invalid {}
impl From<Invalid> for pt_packet {
    fn from(_: Invalid) -> Self {
        pt_packet {
            type_: pt_packet_type_ppt_invalid,
            size: mem::size_of::<pt_packet>() as u8,
            payload: unsafe{ mem::zeroed() }
        }
    }
}

impl Into<Invalid> for pt_packet {
    fn into(self) -> Invalid { Invalid{} }
}