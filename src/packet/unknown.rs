use std::any::Any;
use libipt_sys::pt_packet_unknown;

/// An unknown packet decodable by the optional decoder callback.
/// Packet: unknown
pub struct Unknown (Box<dyn Any>);
impl Unknown {
    /// The custom data you returned from the callback
    pub fn data(&self) -> &dyn Any { &self.0 }
    /// The custom data you returned from the callback
    pub fn data_mut(&mut self) -> &mut dyn Any { &mut self.0 }
}

impl From<pt_packet_unknown> for Unknown {
    fn from(pkt: pt_packet_unknown) -> Self {
        unsafe {
            Unknown(Box::from_raw(pkt.priv_ as *mut dyn Any))
        }
    }
}