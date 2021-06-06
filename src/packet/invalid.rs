use libipt_sys::pt_packet;

#[derive(Clone, Copy, Debug)]
pub struct Invalid {}

impl Into<Invalid> for pt_packet {
    fn into(self) -> Invalid { Invalid{} }
}
