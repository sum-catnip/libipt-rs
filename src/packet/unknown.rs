use std::marker::PhantomData;
use std::slice;
use libipt_sys::pt_packet_unknown;

// TODO: expose custom context field
// maybe make this generic?
// now sure what to do here tbh

/// An unknown packet decodable by the optional decoder callback.
/// Packet: unknown
#[derive(Clone, Copy)]
pub struct Unknown<'a> (pt_packet_unknown, &'a [u8]);
impl<'a> Unknown<'a> {
}
