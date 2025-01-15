use super::Block;
use crate::asid::Asid;
use crate::error::{ensure_ptok, extract_pterr, extract_status_or_pterr, PtError, PtErrorCode};
use crate::event::Event;
use crate::flags::Status;
use crate::image::Image;

use crate::{EncoderDecoderBuilder, PtEncoderDecoder};
use libipt_sys::{
    pt_asid, pt_blk_alloc_decoder, pt_blk_asid, pt_blk_core_bus_ratio, pt_blk_event,
    pt_blk_free_decoder, pt_blk_get_image, pt_blk_get_offset, pt_blk_get_sync_offset, pt_blk_next,
    pt_blk_set_image, pt_blk_sync_backward, pt_blk_sync_forward, pt_blk_sync_set, pt_blk_time,
    pt_block, pt_block_decoder, pt_event,
};
use std::mem;
use std::ptr;
use std::ptr::NonNull;

/// The decoder will work on the buffer defined in a Config, it shall contain
/// raw trace data and remain valid for the lifetime of the decoder.
///
/// The decoder needs to be synchronized before it can be used.
#[derive(Debug)]
pub struct BlockDecoder<'a> {
    inner: OwnedPtBlockDecoder,
    default_image: Image,
    custom_image: Option<&'a mut Image>,
    builder: EncoderDecoderBuilder<Self>,
}

impl PtEncoderDecoder for BlockDecoder<'_> {
    /// Allocate an Intel PT block decoder.
    ///
    /// The decoder will work on the buffer defined in @config,
    /// it shall contain raw trace data and remain valid for the lifetime of the decoder.
    /// The decoder needs to be synchronized before it can be used.
    fn new_from_builder(builder: EncoderDecoderBuilder<Self>) -> Result<Self, PtError> {
        let inner = OwnedPtBlockDecoder::new(&builder)?;
        let default_image = unsafe { Image::from_borrowed_raw(pt_blk_get_image(inner.as_ptr())) }?;

        Ok(Self {
            inner,
            default_image,
            custom_image: None,
            builder,
        })
    }
}

impl<'a> BlockDecoder<'a> {
    /// Return the current address space identifier.
    ///
    /// On success, provides the current address space identifier in @asid.
    /// Returns Asid on success, a `PtError` otherwise.
    pub fn asid(&self) -> Result<Asid, PtError> {
        let mut a: Asid = Default::default();
        unsafe {
            ensure_ptok(pt_blk_asid(
                self.inner.as_ptr(),
                &mut a.0,
                size_of::<pt_asid>(),
            ))?;
        }
        Ok(a)
    }

    /// Return the current core bus ratio.
    ///
    /// On success, provides the current core:bus ratio.
    /// The ratio is defined as core cycles per bus clock cycle.
    /// Returns `NoCbr` if there has not been a CBR packet.
    pub fn core_bus_ratio(&self) -> Result<u32, PtError> {
        let mut cbr: u32 = 0;
        ensure_ptok(unsafe { pt_blk_core_bus_ratio(self.inner.as_ptr(), &mut cbr) }).map(|_| cbr)
    }

    /// Get the next pending event.
    ///
    /// On success, provides the next event, a `StatusFlag` instance and updates the decoder.
    /// Returns `BadQuery` if there is no event.
    pub fn event(&mut self) -> Result<(Event, Status), PtError> {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        let status = extract_status_or_pterr(unsafe {
            pt_blk_event(self.inner.as_ptr(), &mut evt, mem::size_of::<pt_event>())
        })?;
        Ok((Event(evt), status))
    }

    #[must_use]
    pub fn used_builder(&self) -> &EncoderDecoderBuilder<Self> {
        &self.builder
    }

    /// Get the traced image.
    ///
    /// The returned image may be modified as long as @decoder is not running.
    /// Returns the traced image the decoder uses for reading memory.
    pub fn image(&mut self) -> &mut Image {
        if let Some(i) = self.custom_image.as_deref_mut() {
            i
        } else {
            &mut self.default_image
        }
    }

    /// Get the current decoder position.
    ///
    /// Returns the current decoder position.
    /// Returns Nosync if decoder is out of sync.
    pub fn offset(&self) -> Result<u64, PtError> {
        let mut off: u64 = 0;
        ensure_ptok(unsafe { pt_blk_get_offset(self.inner.as_ptr(), &mut off) }).map(|_| off)
    }

    /// Get the position of the last synchronization point.
    ///
    /// Returns Nosync if the decoder is out of sync.
    pub fn sync_offset(&self) -> Result<u64, PtError> {
        let mut off: u64 = 0;
        ensure_ptok(unsafe { pt_blk_get_sync_offset(self.inner.as_ptr(), &mut off) }).map(|_| off)
    }

    /// Determine the next block of instructions.
    ///
    /// On success, provides the next block of instructions in execution order.
    /// Also Returns a `StatusFlag` instance on success.
    /// Returns Eos to indicate the end of the trace stream.
    /// Subsequent calls to next will continue to return Eos until trace is required to determine the next instruction.
    /// Returns `BadContext` if the decoder encountered an unexpected packet.
    /// Returns `BadOpc` if the decoder encountered unknown packets.
    /// Returns `BadPacket` if the decoder encountered unknown packet payloads.
    /// Returns `BadQuery` if the decoder got out of sync.
    /// Returns Eos if decoding reached the end of the Intel PT buffer.
    /// Returns Nomap if the memory at the instruction address can't be read.
    /// Returns Nosync if the decoder is out of sync.
    pub fn decode_next(&mut self) -> Result<(Block, Status), PtError> {
        let mut blk: pt_block = unsafe { mem::zeroed() };
        let status = extract_status_or_pterr(unsafe {
            pt_blk_next(self.inner.as_ptr(), &mut blk, mem::size_of::<pt_block>())
        })?;
        Ok((Block(blk), status))
    }

    /// Set the traced image.
    ///
    /// Sets the image that the decoder uses for reading memory to image.
    /// If image is None, sets the image to the decoder's default image.
    /// Only one image can be active at any time.
    pub fn set_image(&mut self, img: Option<&'a mut Image>) -> Result<(), PtError> {
        match img {
            None => {
                ensure_ptok(unsafe { pt_blk_set_image(self.inner.as_ptr(), ptr::null_mut()) })?;
                self.custom_image = None;
                self.default_image = unsafe { Image::from_borrowed_raw(pt_blk_get_image(self.inner.as_ptr())) }?;
            },
            Some(i) => {
                ensure_ptok(unsafe { pt_blk_set_image(self.inner.as_ptr(), i.inner.as_ptr()) })?;
                self.custom_image = Some(i);
                debug_assert_eq!(
                    unsafe{pt_blk_get_image(self.inner.as_ptr())},
                    self.custom_image.as_ref().unwrap().inner.as_ptr()
                );
            },
        };

        Ok(())
    }

    // /// Return (move) the image and drop the decoder
    // #[must_use]
    // pub fn into_owned_image(self) -> Image {
    //     self.image
    // }

    pub fn sync_backward(&mut self) -> Result<Status, PtError> {
        extract_pterr(unsafe { pt_blk_sync_backward(self.inner.as_ptr()) })
            .map(|s| Status::from_bits(s).unwrap())
    }

    /// Synchronize an Intel PT block decoder.
    ///
    /// Search for the next synchronization point in forward or backward direction.
    /// If the decoder has not been synchronized, yet,
    /// the search is started at the beginning of the trace buffer in case of forward synchronization and at the end of the trace buffer in case of backward synchronization.
    /// Returns `BadOpc` if an unknown packet is encountered.
    /// Returns `BadPacket` if an unknown packet payload is encountered.
    /// Returns Eos if no further synchronization point is found.
    pub fn sync_forward(&mut self) -> Result<Status, PtError> {
        extract_pterr(unsafe { pt_blk_sync_forward(self.inner.as_ptr()) })
            .map(Status::from_bits_or_pterror)?
    }

    /// Manually synchronize an Intel PT block decoder.
    ///
    /// Synchronize @decoder on the syncpoint at @offset. There must be a PSB packet at @offset.
    /// Returns `BadOpc` if an unknown packet is encountered.
    /// Returns `BadPacket` if an unknown packet payload is encountered.
    /// Returns Eos if @offset lies outside of the decoder's trace buffer.
    /// Returns Eos if the decoder reaches the end of its trace buffer.
    /// Returns Nosync if there is no syncpoint at @offset.
    pub fn set_sync(&mut self, offset: u64) -> Result<(), PtError> {
        ensure_ptok(unsafe { pt_blk_sync_set(self.inner.as_ptr(), offset) })
    }

    /// Return the current time.
    ///
    /// On success, provides the time at the last preceding timing packet,
    /// The number of lost mtc packets and
    /// The number of lost cyc packets.
    ///
    /// The time is similar to what a rdtsc instruction would return.
    /// Depending on the configuration, the time may not be fully accurate.
    /// If TSC is not enabled, the time is relative to the last synchronization and can't be used to correlate with other TSC-based time sources.
    /// In this case, `NoTime` is returned and the relative time is provided.
    /// Some timing-related packets may need to be dropped (mostly due to missing calibration or incomplete configuration).
    /// To get an idea about the quality of the estimated time, we record the number of dropped MTC and CYC packets.
    /// Returns `NoTime` if there has not been a TSC packet.
    pub fn time(&mut self) -> Result<(u64, u32, u32), PtError> {
        let mut time: u64 = 0;
        let mut lost_mtc: u32 = 0;
        let mut lost_cyc: u32 = 0;
        ensure_ptok(unsafe {
            pt_blk_time(self.inner.as_ptr(), &mut time, &mut lost_mtc, &mut lost_cyc)
        })
        .map(|_| (time, lost_mtc, lost_cyc))
    }
}

impl Iterator for BlockDecoder<'_> {
    type Item = Result<(Block, Status), PtError>;

    fn next(&mut self) -> Option<Result<(Block, Status), PtError>> {
        match self.decode_next() {
            // eos to stop iterating
            Err(x) if x.code() == PtErrorCode::Eos => None,
            x => Some(x),
        }
    }
}

/// This struct allow us to not implement Drop for `BlockDecoder` and therefore move out Image with
/// `to_owned_image`
#[derive(Debug)]
struct OwnedPtBlockDecoder {
    inner: NonNull<pt_block_decoder>,
}

impl OwnedPtBlockDecoder {
    fn new(builder: &EncoderDecoderBuilder<BlockDecoder>) -> Result<Self, PtError> {
        NonNull::new(unsafe { pt_blk_alloc_decoder(&raw const builder.config) })
            .ok_or(PtError::new(
                PtErrorCode::Internal,
                "Failed to allocate pt_block_decoder",
            ))
            .map(|inner| Self { inner })
    }

    fn as_ptr(&self) -> *mut pt_block_decoder {
        self.inner.as_ptr()
    }
}

impl Drop for OwnedPtBlockDecoder {
    fn drop(&mut self) {
        unsafe { pt_blk_free_decoder(self.inner.as_ptr()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use libipt_sys::{pt_blk_get_config, pt_config};

    #[test]
    fn test_blkdec_alloc() {
        let mut kek = [1u8; 2];
        let builder = BlockDecoder::builder();
        unsafe { builder.buffer_from_raw(kek.as_mut_ptr(), kek.len()) }
            .build()
            .unwrap();
    }

    #[test]
    fn test_blkdec_props() {
        let kek = &mut [1u8; 2];
        let mut builder = BlockDecoder::builder();
        builder = unsafe { builder.buffer_from_raw(kek.as_mut_ptr(), kek.len()) };
        let mut b = builder.build().unwrap();

        let a = b.asid().unwrap();
        assert!(a.cr3().is_none());
        assert!(a.vmcs().is_none());
        assert!(b.core_bus_ratio().is_ok());
        assert!(b.event().is_err());
        let used_builder = b.used_builder();
        unsafe {
            let inner_config = pt_blk_get_config(b.inner.as_ptr());
            let saved_config = &raw const used_builder.config;
            let size = size_of::<pt_config>();
            debug_assert_eq!(
                std::slice::from_raw_parts(inner_config.cast::<u8>(), size),
                std::slice::from_raw_parts(saved_config.cast::<u8>(), size),
                "Rust builder not coherent with libipt C config!"
            )
        }

        // assert!(b.image().name().is_none());
        assert!(b.offset().is_err());
        assert!(b.sync_offset().is_err());
        assert!(b.decode_next().is_err());
        assert!(b.sync_backward().is_err());
        assert!(b.sync_forward().is_err());
        assert!(b.time().is_ok());
    }
}
