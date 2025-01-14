use super::Packet;
use crate::error::{ensure_ptok, PtError, PtErrorCode};

use crate::{EncoderDecoderBuilder, PtEncoderDecoder};
use libipt_sys::{
    pt_packet, pt_packet_decoder, pt_pkt_alloc_decoder, pt_pkt_free_decoder, pt_pkt_get_offset,
    pt_pkt_get_sync_offset, pt_pkt_next, pt_pkt_sync_backward, pt_pkt_sync_forward,
    pt_pkt_sync_set,
};
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct PacketDecoder<T> {
    inner: NonNull<pt_packet_decoder>,
    builder: EncoderDecoderBuilder<Self>,
    phantom: PhantomData<T>,
}

impl<T> PtEncoderDecoder for PacketDecoder<T> {
    /// Allocate an Intel PT packet decoder.
    ///
    /// The decoder will work on the buffer defined in @config,
    /// it shall contain raw trace data and remain valid for the lifetime of the decoder.
    /// The decoder needs to be synchronized before it can be used.
    fn new_from_builder(builder: EncoderDecoderBuilder<Self>) -> Result<Self, PtError> {
        let inner =
            NonNull::new(unsafe { pt_pkt_alloc_decoder(&raw const builder.config) }).ok_or(
                PtError::new(PtErrorCode::Internal, "Failed to allocate pt_insn_decoder"),
            )?;

        Ok(Self {
            inner,
            builder,
            phantom: PhantomData,
        })
    }
}

impl<T> PacketDecoder<T> {
    pub fn used_builder(&self) -> &EncoderDecoderBuilder<Self> {
        &self.builder
    }

    /// Get the current decoder position.
    ///
    /// Returns Nosync if decoder is out of sync.
    pub fn offset(&self) -> Result<u64, PtError> {
        let mut off: u64 = 0;
        ensure_ptok(unsafe { pt_pkt_get_offset(self.inner.as_ptr(), &mut off) }).map(|_| off)
    }

    /// Get the position of the last synchronization point.
    ///
    /// This is useful when splitting a trace stream for parallel decoding.
    /// Returns Nosync if decoder is out of sync.
    pub fn sync_offset(&self) -> Result<u64, PtError> {
        let mut off: u64 = 0;
        ensure_ptok(unsafe { pt_pkt_get_sync_offset(self.inner.as_ptr(), &mut off) }).map(|_| off)
    }

    /// Decode the next packet and advance the decoder.
    ///
    /// Decodes the packet at the decoder's current position and
    /// adjusts the decoder's position by the number of bytes the packet had consumed.
    /// Returns BadOpc if the packet is unknown.
    /// Returns BadPacket if an unknown packet payload is encountered.
    /// Returns Eos if decoder reached the end of the Intel PT buffer.
    /// Returns Nosync if decoder is out of sync.
    pub fn decode_next(&mut self) -> Result<Packet<T>, PtError> {
        let mut pkt: pt_packet = unsafe { mem::zeroed() };
        ensure_ptok(unsafe {
            pt_pkt_next(self.inner.as_ptr(), &mut pkt, mem::size_of::<pt_packet>())
        })
        .map(|_| pkt.into())
    }

    pub fn sync_backward(&mut self) -> Result<(), PtError> {
        ensure_ptok(unsafe { pt_pkt_sync_backward(self.inner.as_ptr()) })
    }

    /// Synchronize an Intel PT packet decoder.
    ///
    /// Search for the next synchronization point in forward or backward direction.
    /// If decoder has not been synchronized, yet, the search is started
    /// at the beginning of the trace buffer in case of forward synchronization
    /// and at the end of the trace buffer in case of backward synchronization.
    /// Returns Eos if no further synchronization point is found.
    pub fn sync_forward(&mut self) -> Result<(), PtError> {
        ensure_ptok(unsafe { pt_pkt_sync_forward(self.inner.as_ptr()) })
    }

    /// Hard set synchronization point of an Intel PT decoder.
    ///
    /// Synchronize decoder to @offset within the trace buffer.
    /// Returns Eos if the given offset is behind the end of the trace buffer.
    pub fn sync_set(&mut self, offset: u64) -> Result<(), PtError> {
        ensure_ptok(unsafe { pt_pkt_sync_set(self.inner.as_ptr(), offset) })
    }
}

impl<T> Iterator for PacketDecoder<T> {
    type Item = Result<Packet<T>, PtError>;

    fn next(&mut self) -> Option<Result<Packet<T>, PtError>> {
        match self.decode_next() {
            // eos to stop iterating
            Err(x) if x.code() == PtErrorCode::Eos => None,
            x => Some(x),
        }
    }
}

impl<T> Drop for PacketDecoder<T> {
    fn drop(&mut self) {
        unsafe { pt_pkt_free_decoder(self.inner.as_ptr()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use libipt_sys::{pt_config, pt_pkt_get_config};

    #[test]
    fn test_pktdec_alloc() {
        let mut kek = [1u8; 2];
        let builder: EncoderDecoderBuilder<PacketDecoder<()>> = PacketDecoder::builder();
        unsafe { builder.buffer_from_raw(kek.as_mut_ptr(), kek.len()) }
            .build()
            .unwrap();
    }

    #[test]
    fn test_pktdec_props() {
        let mut kek = [1u8; 2];
        let builder: EncoderDecoderBuilder<PacketDecoder<()>> = PacketDecoder::builder();
        let mut p = unsafe { builder.buffer_from_raw(kek.as_mut_ptr(), kek.len()) }
            .build()
            .unwrap();

        let used_builder = p.used_builder();
        unsafe {
            let inner_config = pt_pkt_get_config(p.inner.as_ptr());
            let saved_config = &raw const used_builder.config;
            let size = size_of::<pt_config>();
            debug_assert_eq!(
                std::slice::from_raw_parts(inner_config.cast::<u8>(), size),
                std::slice::from_raw_parts(saved_config.cast::<u8>(), size),
                "Rust builder not coherent with libipt C config!"
            )
        }
        assert!(p.offset().is_err());
        assert!(p.sync_offset().is_err());
        assert!(p.decode_next().is_err());
        assert!(p.sync_backward().is_err());
        assert!(p.sync_forward().is_err());
    }
}
