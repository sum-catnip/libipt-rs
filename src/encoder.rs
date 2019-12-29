use super::config::Config;

use super::error::{
    PTError,
    ensure_ptok,
    deref_ptresult
};

use libipt_sys::{
    pt_encoder,
    pt_config,
    pt_alloc_encoder,
    pt_free_encoder,
    pt_enc_get_config,
    pt_enc_get_offset
};

mod tests {
    #[test]
    fn can_allocate_encoder() {

    }
}

pub struct Encoder(pt_encoder);

// TODO deallocate encoder in drop
impl Encoder {
    /// Allocates a new decoder
    pub fn new(cfg: pt_config) -> Result<Encoder, PTError> {
        deref_ptresult(unsafe{pt_alloc_encoder(&cfg)})
            .map(|x| Encoder(*x))
    }

    pub fn config(&self) -> Result<Config, PTError> {
        deref_ptresult(unsafe{pt_enc_get_config(&self.0)})
            .map(Config::from)
    }

    pub fn offset(&self) -> Result<u64, PTError> {
        let mut off = 0;
        ensure_ptok(unsafe{pt_enc_get_offset(&self.0, &mut off)})
            .map(|_| off)
    }
}

impl Drop for Encoder {
    fn drop(&mut self) { unsafe { pt_free_encoder(&mut self.0) } }
}