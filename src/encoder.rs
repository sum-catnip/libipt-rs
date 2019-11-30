use libipt_sys;
use libipt_sys::{pt_encoder, pt_config};

mod tests {
    #[test]
    fn can_allocate_encoder() {

    }
}

pub struct Encoder(*const pt_encoder);

// TODO wrap pt_config and use that here
impl Encoder {
    pub fn new(cfg: pt_config) -> Encoder {
        unsafe {
            Encoder(libipt_sys::pt_alloc_encoder(&cfg))
        }
    }
}