use intelpt_sys;
use intelpt_sys::{pt_encoder, pt_config};

mod tests {
    #[test]
    fn kektop() {
        println!("hi");
    }
}

pub struct Encoder {
    _wrapped: pt_encoder;
}

impl Encoder {
    pub fn new(cfg: pt_config) -> Encoder {

    }
}