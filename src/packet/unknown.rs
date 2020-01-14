use std::any::Any;
use libipt_sys::pt_packet_unknown;

/// An unknown packet decodable by the optional decoder callback.
/// Packet: unknown
pub struct Unknown<'a> (Option<&'a mut dyn Any>);
impl<'a> Unknown<'a> {
    // Create new instance of Unknown, putting `data` in a box
    pub fn new(data: Option<&'a mut dyn Any>) -> Self {
        Unknown(data)
    }

    /*
    /// The custom data you returned from the callback
    pub fn data(&self) -> Option<&'a dyn Any> {
    }

    /// The custom data you returned from the callback
    pub fn data_mut(&mut self) -> Option<&mut dyn Any> {
        self.0.as_mut().map(|x| &mut **x as &mut dyn Any)
    }*/
}

impl<'a> From<pt_packet_unknown> for Unknown<'_> {
    fn from(pkt: pt_packet_unknown) -> Self {
        unsafe {
            assert!(Box::from_raw(pkt.priv_ as *mut dyn Any).is::<String>());
            match pkt.priv_ {
                x if x.is_null() => Unknown(None),
                x => Box::from_raw(x as *mut Unknown)
            }
        }
    }
}
