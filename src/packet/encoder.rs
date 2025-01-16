use crate::error::{ensure_ptok, extract_pterr, PtError};

use crate::config::{EncoderDecoderBuilder,PtEncoderDecoder};
use crate::error::{ PtErrorCode};
use libipt_sys::{
    pt_alloc_encoder, pt_enc_get_offset, pt_enc_next, pt_enc_sync_set, pt_encoder, pt_free_encoder,
    pt_packet,
};
use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Encoder<T> {
    inner: NonNull<pt_encoder>,
    phantom: PhantomData<T>,
}

impl<T> PtEncoderDecoder for Encoder<T> {
    /// Allocate an Intel PT packet encoder.
    ///
    /// The encoder will work on the buffer defined in @config, it shall contain raw trace data and remain valid for the lifetime of the encoder.
    /// The encoder starts at the beginning of the trace buffer.
    fn new_from_builder(builder: EncoderDecoderBuilder<Self>) -> Result<Self, PtError> {
        let inner = NonNull::new(unsafe { pt_alloc_encoder(&raw const builder.config) }).ok_or(
            PtError::new(PtErrorCode::Internal, "Failed to allocate pt_encoder"),
        )?;

        Ok(Self {
            inner,
            phantom: PhantomData,
        })
    }
}

impl<T> Encoder<T> {
    // pub fn config(&self) -> Result<Config<T>, PtError> {
    //     deref_ptresult(unsafe { pt_enc_get_config(self.inner.as_ptr()) }).map(Config::from)
    // }

    /// Get the current packet encoder position.
    ///
    /// Gets the current encoder position
    /// Returns the offset on success, a PtError otherwise
    /// Returns Invalid if @offset is NULL.
    pub fn offset(&self) -> Result<u64, PtError> {
        let mut off = 0;
        ensure_ptok(unsafe { pt_enc_get_offset(self.inner.as_ptr(), &mut off) }).map(|_| off)
    }

    /// Encode an Intel PT packet.
    ///
    /// Writes @packet at the encoder's current position in the Intel PT buffer and advances the encoder beyond the written packet.
    /// In case of errors, the encoder is not advanced and nothing is written into the Intel PT buffer.
    /// Returns the number of bytes written on success, a PtError otherwise
    /// Returns BadOpc if @packet.type is not known.
    /// Returns BadPacket if @packet's payload is invalid.
    /// Returns Eos if the encoder reached the end of the Intel PT buffer.
    pub fn next(&mut self, pck: impl Into<pt_packet>) -> Result<u32, PtError> {
        extract_pterr(unsafe { pt_enc_next(self.inner.as_ptr(), &pck.into()) })
    }

    /// Hard set synchronization point of an Intel PT packet encoder.
    ///
    /// Synchronize the encoder to @offset within the trace buffer.
    /// Returns () on success, a PtError otherwise.
    /// Returns Eos if the given offset is behind the end of the trace buffer.
    /// Returns Invalid if the encoder is NULL.
    pub fn set_offset(&mut self, offset: u64) -> Result<(), PtError> {
        ensure_ptok(unsafe { pt_enc_sync_set(self.inner.as_ptr(), offset) })
    }
}

impl<T> Drop for Encoder<T> {
    fn drop(&mut self) {
        unsafe { pt_free_encoder(self.inner.as_ptr()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::Mnt;

    #[test]
    fn test_pktdec_alloc() {
        let mut kek = [1u8; 2];
        let builder: EncoderDecoderBuilder<Encoder<()>> = Encoder::builder();
        unsafe { builder.buffer_from_raw(kek.as_mut_ptr(), kek.len()) }
            .build()
            .unwrap();
    }

    #[test]
    fn test_pktdec_props() {
        let mut kek = [1u8; 2];
        let builder: EncoderDecoderBuilder<Encoder<()>> = Encoder::builder();
        let mut p = unsafe { builder.buffer_from_raw(kek.as_mut_ptr(), kek.len()) }
            .build()
            .unwrap();

        // assert!(p.config().is_ok());
        assert_eq!(p.offset().unwrap(), 0);
        assert!(p.set_offset(6).is_err());
        assert!(p.next(Mnt::new(5)).is_err());
    }
}
