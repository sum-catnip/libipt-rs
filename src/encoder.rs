use libipt_sys;
use libipt_sys::{pt_encoder, pt_config};

mod tests {
    #[test]
    fn kektop() {
        println!("hi");
    }
}

pub struct Encoder(*const pt_encoder);

// TODO wrap pt_config and use that here
impl Encoder {
    pub fn new(cfg: *const pt_config) -> Encoder {
        unsafe {
            let enc = libipt_sys::pt_alloc_encoder(cfg);
            Encoder (enc)
        }
    }
}