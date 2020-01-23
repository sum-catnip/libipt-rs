use super::Packet;
use crate::error::{PtError, deref_ptresult, ensure_ptok};
use crate::Config;

use std::mem;
use std::marker::PhantomData;

use libipt_sys::{
    pt_packet_decoder,
    pt_pkt_alloc_decoder,
    pt_pkt_free_decoder,
    pt_pkt_get_config,
    pt_pkt_get_offset,
    pt_pkt_get_sync_offset,
    pt_pkt_next,
    pt_packet,
    pt_pkt_sync_backward,
    pt_pkt_sync_forward,
    pt_pkt_sync_set
};

pub struct PacketDecoder<T>(pt_packet_decoder, PhantomData<T>);
impl<T> PacketDecoder<T> {
    /// Allocate an Intel PT packet decoder.
    ///
    /// The decoder will work on the buffer defined in @config,
    /// it shall contain raw trace data and remain valid for the lifetime of the decoder.
    /// The decoder needs to be synchronized before it can be used.
    pub fn new(cfg: &Config<T>) -> Result<Self, PtError> {
        deref_ptresult(unsafe { pt_pkt_alloc_decoder(cfg.0.as_ref()) })
            .map(|d| PacketDecoder::<T>(*d, PhantomData))
    }

    pub fn config(&self) -> Result<Config<T>, PtError> {
        deref_ptresult(unsafe { pt_pkt_get_config(&self.0) })
            .map(Config::from)
    }

    /// Get the current decoder position.
    ///
    /// Returns Nosync if decoder is out of sync.
    pub fn offset(&self) -> Result<u64, PtError> {
        let mut off: u64 = 0;
        ensure_ptok(unsafe { pt_pkt_get_offset(&self.0, &mut off) })
            .map(|_| off)
    }

    /// Get the position of the last synchronization point.
    ///
    /// This is useful when splitting a trace stream for parallel decoding.
    /// Returns Nosync if decoder is out of sync.
    pub fn sync_offset(&self) -> Result<u64, PtError> {
        let mut off: u64 = 0;
        ensure_ptok(unsafe { pt_pkt_get_sync_offset(&self.0, &mut off) })
            .map(|_| off)
    }

    /// Decode the next packet and advance the decoder.
    ///
    /// Decodes the packet at the decoder's current position and
    /// adjusts the decoder's position by the number of bytes the packet had consumed.
    /// Returns BadOpc if the packet is unknown.
    /// Returns BadPacket if an unknown packet payload is encountered.
    /// Returns Eos if decoder reached the end of the Intel PT buffer.
    /// Returns Nosync if decoder is out of sync.
    pub fn next(&mut self) -> Result<Packet<T>, PtError> {
        let mut pkt: pt_packet = unsafe { mem::zeroed() };
        ensure_ptok(unsafe {
            pt_pkt_next(&mut self.0,
                        &mut pkt,
                        mem::size_of::<pt_packet>())
        }).map(|_| pkt.into())
    }

    pub fn sync_backward(&mut self) -> Result<(), PtError> {
        ensure_ptok(unsafe { pt_pkt_sync_backward(&mut self.0) })
    }

    /// Synchronize an Intel PT packet decoder.
    ///
    /// Search for the next synchronization point in forward or backward direction.
    /// If decoder has not been synchronized, yet, the search is started
    /// at the beginning of the trace buffer in case of forward synchronization
    /// and at the end of the trace buffer in case of backward synchronization.
    /// Returns Eos if no further synchronization point is found.
    pub fn sync_forward(&mut self) -> Result<(), PtError> {
        ensure_ptok(unsafe { pt_pkt_sync_forward(&mut self.0) })
    }

    /// Hard set synchronization point of an Intel PT decoder.
    ///
    /// Synchronize decoder to @offset within the trace buffer.
    /// Returns Eos if the given offset is behind the end of the trace buffer.
    pub fn sync_set(&mut self, offset: u64) -> Result<(), PtError> {
        ensure_ptok(unsafe { pt_pkt_sync_set(&mut self.0, offset) })
    }
}

impl<T> Drop for PacketDecoder<T> {
    fn drop(&mut self) { unsafe { pt_pkt_free_decoder(&mut self.0) }}
}