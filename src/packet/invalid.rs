use libipt_sys::pt_packet;

#[derive(Clone, Copy, Debug)]
pub struct Invalid {}

impl From<pt_packet> for Invalid {
    fn from(_val: pt_packet) -> Self { Invalid{} }
}
