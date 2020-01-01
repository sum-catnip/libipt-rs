use std::mem;
use crate::asid::Asid;
use crate::config::Config;
use crate::error::{PtError, ensure_ptok, deref_ptresult};
use crate::event::Event;
use crate::flags::Status;
use crate::image::Image;

use libipt_sys::{
    pt_packet,
    pt_event,
    pt_block_decoder,
    pt_blk_alloc_decoder,
    pt_blk_core_bus_ratio,
    pt_blk_free_decoder,
    pt_blk_get_config,
    pt_blk_get_image,
    pt_blk_get_offset,
    pt_blk_event,
    pt_blk_asid,
    pt_asid
};

pub struct Block(pt_block_decoder);
impl Block {
    /// Allocate an Intel PT block decoder.
    ///
    /// The decoder will work on the buffer defined in @config,
    /// it shall contain raw trace data and remain valid for the lifetime of the decoder.
    /// The decoder needs to be synchronized before it can be used.
    pub fn new(cfg: &Config) -> Result<Block, PtError> {
        deref_ptresult(unsafe{pt_blk_alloc_decoder(&cfg.0)})
            .map(|x| Block(*x))
    }

    /// Return the current address space identifier.
    ///
    /// On success, provides the current address space identifier in @asid.
    /// Returns Asid on success, a PtError otherwise.
    pub fn asid(&self) -> Result<Asid, PtError> {
        let mut a = Asid::new(0, 0);
        unsafe {
            ensure_ptok(pt_blk_asid(&self.0, &mut a.0, mem::size_of::<pt_asid>()))
                .map(|_| a)
        }
    }

    /// Return the current core bus ratio.
    ///
    /// On success, provides the current core:bus ratio.
    /// The ratio is defined as core cycles per bus clock cycle.
    /// Returns NoCbr if there has not been a CBR packet.
    pub fn core_bus_ratio(&mut self) -> Result<u32, PtError> {
        let mut cbr: u32 = 0;
        unsafe { ensure_ptok(pt_blk_core_bus_ratio(&mut self.0, &mut cbr)) }
    }

    /// Get the next pending event.
    ///
    /// On success, provides the next event, a StatusFlag instance and updates the decoder.
    /// Returns BadQuery if there is no event.
    pub fn event(&mut self) -> Result<(Event, Status), PtError> {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        let status = ensure_ptok(unsafe { pt_blk_event(
            &mut self.0, &mut evt, mem::size_of::<pt_event>()
        )})?;
        Ok((Event(evt), Status::from_bits(status).unwrap()))
    }

    pub fn config(&self) -> Result<Config, PtError> {
        deref_ptresult( unsafe { pt_blk_get_config(&self.0) })
            .map(Config::from)
    }

    /// Get the traced image.
    ///
    /// The returned image may be modified as long as @decoder is not running.
    /// Returns the traced image the decoder uses for reading memory.
    pub fn image(&mut self) -> Result<Image, PtError> {
        deref_ptresult( unsafe { pt_blk_get_image(&mut self.0) })
            .map(|i| Image(*i))
    }

    /// Get the current decoder position.
    ///
    /// Returns the current decoder position.
    /// Returns Nosync if decoder is out of sync.
    pub fn offset(&self) -> Result<u64, PtError> {
        let mut off: u64 = 0;
        ensure_ptok( unsafe { pt_blk_get_offset(&self.0, &mut off) })
            .map(|_| off)
    }
}

impl Drop for Block {
    fn drop(&mut self) { unsafe { pt_blk_free_decoder(&mut self.0) } }
}