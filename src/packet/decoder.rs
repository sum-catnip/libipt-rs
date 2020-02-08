use crate::error::{
    PtError, PtErrorCode,
    deref_ptresult, deref_ptresult_mut,
    ensure_ptok
};
use super::Packet;
use crate::config::Config;

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::ConfigBuilder;

    #[test]
    fn test_pktdec_alloc() {
        let daturu = &mut [11; 11];
        PacketDecoder::new(&ConfigBuilder::new(daturu)
            .unwrap()
            .finish()
        ).unwrap();
    }

    #[test ]
    fn test_pktdec_props() {
        let daturu = &mut [11; 11];
        // this just checks memory safety for property access
        // usage can be found in the integration tests
        let mut p = PacketDecoder::new(
            &ConfigBuilder::new(daturu).unwrap().finish()
        ).unwrap();
        assert!(p.config().is_ok());
        assert!(p.offset().is_err());
        assert!(p.sync_offset().is_err());
        assert!(p.next().is_err());
        assert!(p.sync_backward().is_err());
        assert!(p.sync_forward().is_err());
    }
}

pub struct PacketDecoder<'a, T>(&'a mut pt_packet_decoder, PhantomData<T>);
impl<'a, T> PacketDecoder<'a, T> {
    /// Allocate an Intel PT packet decoder.
    ///
    /// The decoder will work on the buffer defined in @config,
    /// it shall contain raw trace data and remain valid for the lifetime of the decoder.
    /// The decoder needs to be synchronized before it can be used.
    pub fn new(cfg: &Config<T>) -> Result<Self, PtError> {
        deref_ptresult_mut(unsafe { pt_pkt_alloc_decoder(cfg.0.as_ref()) })
            .map(|d| PacketDecoder::<T>(d, PhantomData))
    }

    pub fn config(&self) -> Result<Config<T>, PtError> {
        deref_ptresult(unsafe { pt_pkt_get_config(self.0) })
            .map(Config::from)
    }

    /// Get the current decoder position.
    ///
    /// Returns Nosync if decoder is out of sync.
    pub fn offset(&self) -> Result<u64, PtError> {
        let mut off: u64 = 0;
        ensure_ptok(unsafe { pt_pkt_get_offset(self.0, &mut off) })
            .map(|_| off)
    }

    /// Get the position of the last synchronization point.
    ///
    /// This is useful when splitting a trace stream for parallel decoding.
    /// Returns Nosync if decoder is out of sync.
    pub fn sync_offset(&self) -> Result<u64, PtError> {
        let mut off: u64 = 0;
        ensure_ptok(unsafe { pt_pkt_get_sync_offset(self.0, &mut off) })
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
            pt_pkt_next(self.0,
                        &mut pkt,
                        mem::size_of::<pt_packet>())
        }).map(|_| pkt.into())
    }

    pub fn sync_backward(&mut self) -> Result<(), PtError> {
        ensure_ptok(unsafe { pt_pkt_sync_backward(self.0) })
    }

    /// Synchronize an Intel PT packet decoder.
    ///
    /// Search for the next synchronization point in forward or backward direction.
    /// If decoder has not been synchronized, yet, the search is started
    /// at the beginning of the trace buffer in case of forward synchronization
    /// and at the end of the trace buffer in case of backward synchronization.
    /// Returns Eos if no further synchronization point is found.
    pub fn sync_forward(&mut self) -> Result<(), PtError> {
        ensure_ptok(unsafe { pt_pkt_sync_forward(self.0) })
    }

    /// Hard set synchronization point of an Intel PT decoder.
    ///
    /// Synchronize decoder to @offset within the trace buffer.
    /// Returns Eos if the given offset is behind the end of the trace buffer.
    pub fn sync_set(&mut self, offset: u64) -> Result<(), PtError> {
        ensure_ptok(unsafe { pt_pkt_sync_set(self.0, offset) })
    }
}

impl<'a, T> Iterator for PacketDecoder<'a, T> {
    type Item = Result<Packet<T>, PtError>;

    fn next(&mut self) -> Option<Result<Packet<T>, PtError>> {
        match self.next() {
            // eos to stop iterating
            Err(x) if x.code() == PtErrorCode::Eos => None,
            x => Some(x)
        }
    }
}

impl<'a, T> Drop for PacketDecoder<'a, T> {
    fn drop(&mut self) { unsafe { pt_pkt_free_decoder(self.0) }}
}
