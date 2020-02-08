use crate::error::{
    PtError, deref_ptresult,
    deref_ptresult_mut, PtErrorCode,
    ensure_ptok, extract_pterr
};
use crate::config::Config;
use crate::Asid;
use crate::event::Event;
use crate::Status;
use crate::Image;
use super::Insn;

use std::mem;
use std::ptr;
use std::marker::PhantomData;

use libipt_sys::{
    pt_insn_decoder,
    pt_insn_alloc_decoder,
    pt_insn_asid,
    pt_asid,
    pt_insn_core_bus_ratio,
    pt_insn_event,
    pt_event,
    pt_insn_free_decoder,
    pt_insn_get_config,
    pt_insn_get_image,
    pt_insn_get_offset,
    pt_insn_get_sync_offset,
    pt_insn_next,
    pt_insn,
    pt_insn_set_image,
    pt_insn_sync_backward,
    pt_insn_sync_forward,
    pt_insn_sync_set,
    pt_insn_time
};

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::ConfigBuilder;

    #[test]
    fn test_insndec_alloc() {
        let kek = &mut [1; 2];
        InsnDecoder::new(&ConfigBuilder::new(kek).unwrap().finish())
            .unwrap();
    }

    #[test ]
    fn test_insndec_props() {
        let kek = &mut [1; 2];
        // this just checks memory safety for property access
        // usage can be found in the integration tests
        let mut b = InsnDecoder::new(
            &ConfigBuilder::new(kek).unwrap().finish()
        ).unwrap();

        let a = b.asid().unwrap();
        assert!(a.cr3().is_none());
        assert!(a.vmcs().is_none());

        assert!(b.event().is_err());
        assert!(b.core_bus_ratio().is_err());
        assert!(b.event().is_err());
        assert!(b.config().is_ok());
        assert!(b.image().unwrap().name().is_none());
        assert!(b.offset().is_err());
        assert!(b.sync_offset().is_err());
        assert!(b.next().is_err());
        assert!(b.sync_backward().is_err());
        assert!(b.sync_forward().is_err());
        assert!(b.time().is_err());
    }
}

/// The decoder will work on the buffer defined in the Config,
/// it shall contain raw trace data and remain valid for the lifetime of the decoder.
/// The decoder needs to be synchronized before it can be used.
pub struct InsnDecoder<'a, T>(&'a mut pt_insn_decoder, PhantomData<T>);
impl<'a, T> InsnDecoder<'a, T> {
    /// Allocate an Intel PT instruction flow decoder.
    ///
    /// The decoder will work on the buffer defined in @config,
    /// it shall contain raw trace data and remain valid for the lifetime of the decoder.
    /// The decoder needs to be synchronized before it can be used.
    pub fn new(cfg: &Config<T>) -> Result<Self, PtError> {
        deref_ptresult_mut(unsafe { pt_insn_alloc_decoder(cfg.0.as_ref()) })
            .map(|d| InsnDecoder::<T>(d, PhantomData))
    }

    /// Return the current address space identifier.
    pub fn asid(&self) -> Result<Asid, PtError> {
        let mut asid: pt_asid = unsafe { mem::zeroed() };
        ensure_ptok(unsafe {
            pt_insn_asid(self.0,
                         &mut asid,
                         mem::size_of::<pt_asid>())
        }).map(|_| Asid(asid))
    }

    /// Return the current core bus ratio.
    ///
    /// On success, provides the current core:bus ratio
    /// The ratio is defined as core cycles per bus clock cycle.
    /// Returns NoCbr if there has not been a CBR packet.
    pub fn core_bus_ratio(&mut self) -> Result<u32, PtError> {
        let mut cbr: u32 = 0;
        ensure_ptok(unsafe { pt_insn_core_bus_ratio(self.0, &mut cbr) })
            .map(|_| cbr)
    }

    /// Get the next pending event.
    ///
    /// On success, provides the next event with StatusFlag and updates the decoder.
    /// Returns BadQuery if there is no event.
    pub fn event(&mut self) -> Result<(Event, Status), PtError> {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        extract_pterr(unsafe {
            pt_insn_event(self.0,
                          &mut evt,
                          mem::size_of::<pt_event>())
        }).map(|s| (Event(evt), Status::from_bits(s).unwrap()))
    }

    pub fn config(&self) -> Result<Config<T>, PtError> {
        deref_ptresult(unsafe { pt_insn_get_config(self.0) })
            .map(Config::from)
    }

    /// Get the traced image.
    ///
    /// The returned image may be modified as long as no decoder that uses this image is running.
    /// Returns the traced image the decoder uses for reading memory.
    pub fn image(&mut self) -> Result<Image, PtError> {
        deref_ptresult_mut(unsafe { pt_insn_get_image(self.0) })
            .map(Image::from)
    }

    /// Get the current decoder position.
    ///
    /// Returns Nosync if decoder is out of sync.
    pub fn offset(&self) -> Result<u64, PtError> {
        let mut off: u64 = 0;
        ensure_ptok(unsafe { pt_insn_get_offset(self.0, &mut off) })
            .map(|_| off)
    }

    /// Get the position of the last synchronization point.
    ///
    /// Returns Nosync if @decoder is out of sync.
    pub fn sync_offset(&self) -> Result<u64, PtError> {
        let mut off = 0;
        ensure_ptok(unsafe { pt_insn_get_sync_offset(self.0, &mut off) })
            .map(|_| off)
    }

    /// Determine the next instruction.
    ///
    /// On success, provides the next instruction in execution order along with StatusFlags.
    /// Returns Eos to indicate the end of the trace stream.
    /// Subsequent calls to next() will continue to return Eos until trace is required to determine the next instruction.
    /// Returns BadContext if the decoder encountered an unexpected packet.
    /// Returns BadOpc if the decoder encountered unknown packets.
    /// Returns BadPacket if the decoder encountered unknown packet payloads.
    /// Returns BadQuery if the decoder got out of sync.
    /// Returns Eos if decoding reached the end of the Intel PT buffer.
    /// Returns Nomap if the memory at the instruction address can't be read.
    /// Returns Nosync if decoder is out of sync.
    pub fn next(&mut self) -> Result<(Insn, Status), PtError> {
        let mut insn: pt_insn = unsafe { mem::zeroed() };
        extract_pterr(unsafe {
            pt_insn_next(self.0,
                         &mut insn,
                         mem::size_of::<pt_insn>())
        }).map(|s| (Insn(insn), Status::from_bits(s).unwrap()))
    }

    /// Set the traced image.
    ///
    /// Sets the image that the decoder uses for reading memory to @image.
    /// If @image is None, sets the image to decoder's default image.
    /// Only one image can be active at any time.
    pub fn set_image(&mut self, img: Option<&mut Image>) -> Result<(), PtError> {
        ensure_ptok(unsafe {
            pt_insn_set_image(self.0,
                             match img {
                                 None => ptr::null_mut(),
                                 Some(i) => i.inner
                             })
        })
    }

    pub fn sync_backward(&mut self) -> Result<Status, PtError> {
        extract_pterr(unsafe { pt_insn_sync_backward(self.0) })
            .map(|s| Status::from_bits(s).unwrap())
    }

    /// Synchronize an Intel PT instruction flow decoder.
    ///
    /// Search for the next synchronization point in forward or backward direction.
    /// If decoder has not been synchronized, yet,
    /// the search is started at the beginning of the trace buffer
    /// in case of forward synchronization and at the end of the trace buffer
    /// in case of backward synchronization.
    /// Returns BadOpc if an unknown packet is encountered.
    /// Returns BadPacket if an unknown packet payload is encountered.
    /// Returns Eos if no further synchronization point is found.
    pub fn sync_forward(&mut self) -> Result<Status, PtError> {
        extract_pterr(unsafe { pt_insn_sync_forward(self.0) })
            .map(|s| Status::from_bits(s).unwrap())
    }

    /// Manually synchronize an Intel PT instruction flow decoder.
    ///
    /// Synchronize @decoder on the syncpoint at @offset.
    /// There must be a PSB packet at @offset.
    /// Returns BadOpc if an unknown packet is encountered.
    /// Returns BadPacket if an unknown packet payload is encountered.
    /// Returns Eos if @offset lies outside of decoder's trace buffer.
    /// Returns Eos if decoder reaches the end of its trace buffer.
    /// Returns Nosync if there is no syncpoint at @offset.
    pub fn sync_set(&mut self, offset: u64) -> Result<(), PtError> {
        ensure_ptok(unsafe { pt_insn_sync_set(self.0, offset) })
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
    /// In this case, NoTime is returned and the relative time is provided in @time.
    /// Some timing-related packets may need to be dropped (mostly due to missing calibration or incomplete configuration).
    /// To get an idea about the quality of the estimated time, we record the number of dropped MTC and CYC packets.
    /// Returns NoTime if there has not been a TSC packet.
    pub fn time(&mut self) -> Result<(u64, u32, u32), PtError> {
        let mut time: u64 = 0;
        let mut lost_mtc: u32 = 0;
        let mut lost_cyc: u32 = 0;
        ensure_ptok(
            unsafe {
                pt_insn_time(self.0,
                             &mut time,
                             &mut lost_mtc,
                             &mut lost_cyc)
            }
        ).map(|_| (time, lost_mtc, lost_cyc))
    }
}

impl<'a, T> Iterator for InsnDecoder<'a, T> {
    type Item = Result<(Insn, Status), PtError>;

    fn next(&mut self) -> Option<Result<(Insn, Status), PtError>> {
        match self.next() {
            // eos to stop iterating
            Err(x) if x.code() == PtErrorCode::Eos => None,
            x => Some(x)
        }
    }
}

impl<'a, T> Drop for InsnDecoder<'a, T> {
    fn drop(&mut self) { unsafe { pt_insn_free_decoder(self.0) } }
}