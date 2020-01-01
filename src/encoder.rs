use super::config::Config;
use super::error::{PtError, ensure_ptok, deref_ptresult};

use libipt_sys::{
    pt_packet,
    pt_encoder,
    pt_config,
    pt_alloc_encoder,
    pt_free_encoder,
    pt_enc_get_config,
    pt_enc_get_offset,
    pt_enc_next,
    pt_enc_sync_set
};

mod tests {
    #[test]
    fn can_allocate_encoder() {

    }
}

pub struct Encoder(pt_encoder);
impl Encoder {
    /// Allocate an Intel PT packet encoder.
    ///
    /// The encoder will work on the buffer defined in @config, it shall contain raw trace data and remain valid for the lifetime of the encoder.
    /// The encoder starts at the beginning of the trace buffer.
    pub fn new(cfg: &Config) -> Result<Encoder, PtError> {
        deref_ptresult(unsafe{pt_alloc_encoder(&cfg.0)})
            .map(|x| Encoder(*x))
    }

    pub fn config(&self) -> Result<Config, PtError> {
        deref_ptresult(unsafe{pt_enc_get_config(&self.0)})
            .map(Config::from)
    }

    /// Get the current packet encoder position.
    ///
    /// Gets the current encoder position
    /// Returns the offset on success, a PtError otherwise
    /// Returns Invalid if @offset is NULL.
    pub fn offset(&self) -> Result<u64, PtError> {
        let mut off = 0;
        ensure_ptok(unsafe{pt_enc_get_offset(&self.0, &mut off)})
            .map(|_| off)
    }

    /// Encode an Intel PT packet.
    ///
    /// Writes @packet at the encoder's current position in the Intel PT buffer and advances the encoder beyond the written packet.
    /// The @packet.size field is ignored.
    /// In case of errors, the encoder is not advanced and nothing is written into the Intel PT buffer.
    /// Returns the number of bytes written on success, a negative PtError otherwise
    /// Returns BadOpc if @packet.type is not known.
    /// Returns BadPacket if @packet's payload is invalid.
    /// Returns Eos if the encoder reached the end of the Intel PT buffer.
    /// Returns Invalid if @packet is NULL.
    pub fn next(&mut self, pck: impl Into<pt_packet>) -> Result<u32, PtError> {
        ensure_ptok(unsafe{pt_enc_next(&mut self.0, &pck.into())})
    }

    /// Hard set synchronization point of an Intel PT packet encoder.
    ///
    /// Synchronize the encoder to @offset within the trace buffer.
    /// Returns () on success, a PtError otherwise.
    /// Returns Eos if the given offset is behind the end of the trace buffer.
    /// Returns Invalid if the encoder is NULL.
    pub fn set_offset(&mut self, offset: u64) -> Result<(), PtError> {
        ensure_ptok(unsafe{pt_enc_sync_set(&mut self.0, offset)})
            .map(|_|()) // maybe there is a better way to discard the result
    }
}

impl Drop for Encoder {
    fn drop(&mut self) { unsafe { pt_free_encoder(&mut self.0) } }
}