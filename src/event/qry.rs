use crate::enc_dec_builder::{EncoderDecoderBuilder, PtEncoderDecoder};
use crate::error::{ensure_ptok, extract_status_or_pterr, PtError, PtErrorCode};
use crate::event::Event;
use crate::status::Status;

use libipt_sys::{
    pt_event, pt_qry_alloc_decoder, pt_qry_cond_branch, pt_qry_core_bus_ratio, pt_qry_event,
    pt_qry_free_decoder, pt_qry_get_config, pt_qry_get_offset, pt_qry_get_sync_offset,
    pt_qry_indirect_branch, pt_qry_sync_backward, pt_qry_sync_forward, pt_qry_sync_set,
    pt_qry_time, pt_query_decoder,
};
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(i32)]
pub enum CondBranch {
    Taken = 1,
    NotTaken = 0,
}

/// The decoder will work on the buffer defined in the config,
/// it shall contain raw trace data and remain valid for the lifetime of the decoder.
/// The decoder needs to be synchronized before it can be used.
#[derive(Debug)]
#[repr(transparent)]
pub struct QueryDecoder<T> {
    inner: NonNull<pt_query_decoder>,
    phantom: PhantomData<T>,
}

impl<T> PtEncoderDecoder for QueryDecoder<T> {
    /// Allocate an Intel PT query decoder.
    ///
    /// The decoder will work on the buffer defined in @config,
    /// it shall contain raw trace data and remain valid for the lifetime of the decoder.
    /// The decoder needs to be synchronized before it can be used.
    fn new_from_builder(builder: &EncoderDecoderBuilder<Self>) -> Result<Self, PtError> {
        let inner =
            NonNull::new(unsafe { pt_qry_alloc_decoder(&raw const builder.config) }).ok_or(
                PtError::new(PtErrorCode::Internal, "Failed to allocate pt_query_decoder"),
            )?;
        Ok(Self {
            inner,
            phantom: PhantomData,
        })
    }
}

impl<T> QueryDecoder<T> {
    /// Query whether the next unconditional branch has been taken.
    ///
    /// On success, provides Taken or NotTaken along with StatusFlags
    /// for the next conditional branch and updates decoder.
    /// Returns BadOpc if an unknown packet is encountered.
    /// Returns BadPacket if an unknown packet payload is encountered.
    /// Returns BadQuery if no conditional branch is found.
    /// Returns Eos if decoding reached the end of the Intel PT buffer.
    /// Returns Nosync if decoder is out of sync.
    pub fn cond_branch(&mut self) -> Result<(CondBranch, Status), PtError> {
        let mut taken: i32 = 0;
        let status = extract_status_or_pterr(unsafe {
            pt_qry_cond_branch(self.inner.as_ptr(), &mut taken)
        })?;
        let cond_branch = CondBranch::try_from(taken)?;
        Ok((cond_branch, status))
    }

    /// Return the current core bus ratio.
    ///
    /// On success, provides the current core:bus ratio
    /// The ratio is defined as core cycles per bus clock cycle.
    /// Returns NoCbr if there has not been a CBR packet.
    pub fn core_bus_ratio(&self) -> Result<u32, PtError> {
        let mut cbr: u32 = 0;
        ensure_ptok(unsafe { pt_qry_core_bus_ratio(self.inner.as_ptr(), &mut cbr) }).map(|_| cbr)
    }

    /// Query the next pending event.
    ///
    /// On success, provides the next event along with its status and updates the decoder.
    /// Returns BadOpc if an unknown packet is encountered.
    /// Returns BadPacket if an unknown packet payload is encountered.
    /// Returns BadQuery if no event is found.
    /// Returns Eos if decoding reached the end of the Intel PT buffer.
    /// Returns Nosync if decoder is out of sync.
    pub fn event(&mut self) -> Result<(Event, Status), PtError> {
        let mut evt = MaybeUninit::<pt_event>::uninit();
        let status = extract_status_or_pterr(unsafe {
            pt_qry_event(self.inner.as_ptr(), evt.as_mut_ptr(), size_of::<pt_event>())
        })?;
        Ok((Event(unsafe { evt.assume_init() }), status))
    }

    #[must_use]
    pub fn used_builder(&self) -> &EncoderDecoderBuilder<Self> {
        let ptr = unsafe { pt_qry_get_config(self.inner.as_ptr()) };
        // The returned pointer is NULL if their argument is NULL. It should never happen.
        unsafe { ptr.cast::<EncoderDecoderBuilder<Self>>().as_ref() }
            .expect("pt_qry_get_config returned a NULL pointer")
    }

    /// Get the current decoder position.
    ///
    /// Returns Nosync if decoder is out of sync.
    pub fn offset(&self) -> Result<u64, PtError> {
        let mut off: u64 = 0;
        ensure_ptok(unsafe { pt_qry_get_offset(self.inner.as_ptr(), &mut off) }).map(|_| off)
    }

    /// Get the position of the last synchronization point.
    ///
    /// This is useful for splitting a trace stream for parallel decoding.
    /// Returns Nosync if decoder is out of sync.
    pub fn sync_offset(&self) -> Result<u64, PtError> {
        let mut off: u64 = 0;
        ensure_ptok(unsafe { pt_qry_get_sync_offset(self.inner.as_ptr(), &mut off) }).map(|_| off)
    }

    /// Get the next indirect branch destination.
    ///
    /// On success, provides the linear destination address
    /// of the next indirect branch along with the status
    /// and updates the decoder.
    /// Returns `BadOpc` if an unknown packet is encountered.
    /// Returns `BadPacket` if an unknown packet payload is encountered.
    /// Returns `BadQuery` if no indirect branch is found.
    /// Returns Eos if decoding reached the end of the Intel PT buffer.
    /// Returns Nosync if decoder is out of sync.
    pub fn indirect_branch(&mut self) -> Result<(u64, Status), PtError> {
        let mut ip: u64 = 0;
        let status = extract_status_or_pterr(unsafe {
            pt_qry_indirect_branch(self.inner.as_ptr(), &mut ip)
        })?;
        Ok((ip, status))
    }

    /// Synchronize an Intel PT query decoder.
    ///
    /// Search for the next synchronization point in forward or backward direction.
    /// If decoder has not been synchronized, yet, the search is started at the beginning
    /// of the trace buffer in case of forward synchronization
    /// and at the end of the trace buffer in case of backward synchronization.
    /// Returns the last ip along with a non-negative Status on success
    /// Returns `BadOpc` if an unknown packet is encountered.
    /// Returns `BadPacket` if an unknown packet payload is encountered.
    /// Returns Eos if no further synchronization point is found.
    pub fn sync_backward(&mut self) -> Result<(u64, Status), PtError> {
        let mut ip: u64 = 0;
        let status =
            extract_status_or_pterr(unsafe { pt_qry_sync_backward(self.inner.as_ptr(), &mut ip) })?;
        Ok((ip, status))
    }

    /// Synchronize an Intel PT query decoder.
    ///
    /// Search for the next synchronization point in forward or backward direction.
    /// If decoder has not been synchronized, yet, the search is started at the beginning
    /// of the trace buffer in case of forward synchronization
    /// and at the end of the trace buffer in case of backward synchronization.
    /// Returns the last ip along with a non-negative Status on success
    /// Returns `BadOpc` if an unknown packet is encountered.
    /// Returns `BadPacket` if an unknown packet payload is encountered.
    /// Returns Eos if no further synchronization point is found.
    pub fn sync_forward(&mut self) -> Result<(u64, Status), PtError> {
        let mut ip: u64 = 0;
        let status =
            extract_status_or_pterr(unsafe { pt_qry_sync_forward(self.inner.as_ptr(), &mut ip) })?;
        Ok((ip, status))
    }

    /// Manually synchronize an Intel PT query decoder.
    ///
    /// Synchronize decoder on the syncpoint at @offset.
    /// There must be a PSB packet at @offset.
    /// Returns last ip along with a status.
    /// Returns `BadOpc` if an unknown packet is encountered.
    /// Returns `BadPacket` if an unknown packet payload is encountered.
    /// Returns Eos if @offset lies outside of decoder's trace buffer.
    /// Returns Eos if decoder reaches the end of its trace buffer.
    /// Returns Nosync if there is no syncpoint at @offset.
    pub fn sync_set(&mut self, offset: u64) -> Result<(u64, Status), PtError> {
        let mut ip: u64 = 0;
        let status = extract_status_or_pterr(unsafe {
            pt_qry_sync_set(self.inner.as_ptr(), &mut ip, offset)
        })?;
        Ok((ip, status))
    }

    /// Query the current time.
    ///
    /// On success, provides the time at the last query.
    /// The time is similar to what a rdtsc instruction would return.
    /// Depending on the configuration, the time may not be fully accurate.
    /// If TSC is not enabled, the time is relative to the last synchronization
    /// and can't be used to correlate with other TSC-based time sources.
    /// In this case, `NoTime` is returned and the relative time is provided.
    /// Some timing-related packets may need to be dropped (mostly due to missing calibration or incomplete configuration).
    /// To get an idea about the quality of the estimated time, we record the number of dropped MTC and CYC packets.
    /// Returns time, number of lost mtc packets and number of lost cyc packets.
    /// Returns `NoTime` if there has not been a TSC packet.
    pub fn time(&mut self) -> Result<(u64, u32, u32), PtError> {
        let mut time: u64 = 0;
        let mut mtc: u32 = 0;
        let mut cyc: u32 = 0;
        ensure_ptok(unsafe { pt_qry_time(self.inner.as_ptr(), &mut time, &mut mtc, &mut cyc) })
            .map(|_| (time, mtc, cyc))
    }
}

impl<T> Iterator for QueryDecoder<T> {
    type Item = Result<(Event, Status), PtError>;

    fn next(&mut self) -> Option<Result<(Event, Status), PtError>> {
        match self.event() {
            // eos to stop iterating
            Err(x) if x.code() == PtErrorCode::Eos => None,
            x => Some(x),
        }
    }
}

impl<T> Drop for QueryDecoder<T> {
    fn drop(&mut self) {
        unsafe { pt_qry_free_decoder(self.inner.as_ptr()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use libipt_sys::pt_config;

    #[test]
    fn test_qrydec_alloc() {
        let mut kek = [1u8; 2];
        let builder: EncoderDecoderBuilder<QueryDecoder<()>> = QueryDecoder::builder();
        unsafe { builder.buffer_from_raw(kek.as_mut_ptr(), kek.len()) }
            .build()
            .unwrap();
    }

    #[test]
    fn test_qrydec_props() {
        let mut kek = [1u8; 2];
        let builder: EncoderDecoderBuilder<QueryDecoder<()>> = QueryDecoder::builder();
        let mut b = unsafe { builder.buffer_from_raw(kek.as_mut_ptr(), kek.len()) }
            .build()
            .unwrap();

        assert!(b.cond_branch().is_err());
        assert!(b.indirect_branch().is_err());
        assert!(b.event().is_err());
        assert!(b.core_bus_ratio().is_err());
        assert!(b.event().is_err());
        let used_builder = b.used_builder();
        unsafe {
            let inner_config = pt_qry_get_config(b.inner.as_ptr());
            let saved_config = &raw const used_builder.config;
            let size = size_of::<pt_config>();
            debug_assert_eq!(
                std::slice::from_raw_parts(inner_config.cast::<u8>(), size),
                std::slice::from_raw_parts(saved_config.cast::<u8>(), size),
                "Rust builder not coherent with libipt C config!"
            )
        }
        assert!(b.offset().is_err());
        assert!(b.sync_offset().is_err());
        assert!(b.sync_backward().is_err());
        assert!(b.sync_forward().is_err());
        assert!(b.time().is_err());
    }
}
