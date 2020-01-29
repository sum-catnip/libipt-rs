use libipt_sys::pt_packet_unknown;

/// An unknown packet decodable by the optional decoder callback.
/// Packet: unknown
pub struct Unknown<T> (pub(crate) Option<Box<T>>);
impl<T> Unknown<T> {
    // Create new instance of Unknown, putting `data` in a box
    pub fn new(data: T) -> Self {
        Unknown(Some(Box::new(data)))
    }

    // Create an empty Unknown, for when you dont want to return something
    pub fn none() -> Self { Unknown(None) }

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
}