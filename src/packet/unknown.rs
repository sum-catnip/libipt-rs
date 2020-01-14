use libipt_sys::pt_packet_unknown;

/// An unknown packet decodable by the optional decoder callback.
/// Packet: unknown
pub struct Unknown<T> (pub(crate) Option<Box<T>>);
impl<T> Unknown<T> {
    // Create new instance of Unknown, putting `data` in a box
    pub fn new(data: Option<T>) -> Self {
        Unknown(data.map(|x| Box::new(x)))
    }

    pub(crate) fn from(pkt: pt_packet_unknown) -> Self {
        match pkt.priv_ {
            x if x.is_null() => Unknown(None),
            x => unsafe {
                Unknown(Some(Box::from_raw(x as *mut T)))
            }
        }
    }

    /// The custom data you returned from the callback
    pub fn data(self) -> Option<T> {
        self.0.map(|d| *d)
    }

    /*
    /// The custom data you returned from the callback
    pub fn data_mut(&mut self) -> Option<&mut dyn Any> {
        self.0.as_mut().map(|x| &mut **x as &mut dyn Any)
    }*/
}