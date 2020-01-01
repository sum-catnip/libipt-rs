use std::mem;
use crate::asid::Asid;
use crate::config::Config;
use crate::error::{PtError, ensure_ptok, deref_ptresult};

use libipt_sys::{
    pt_packet,
    pt_block_decoder,
    pt_blk_alloc_decoder,
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
}