use libipt_sys::pt_packet_unknown;

/// An unknown packet decodable by the optional decoder callback.
/// Packet: unknown
#[derive(Clone, Copy)]
pub struct Unknown (pt_packet_unknown);