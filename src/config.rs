use libipt_sys::{pt_config};

pub struct Config {
    _wrapped: pt_config,
    buffer: Vec<u8>
}

impl Config {
    /// the simples constructor
    /// offering none of the intelpt configuration options
    pub fn new(mut buffer: Vec<u8>) -> Config {
        let cfg = pt_config {
            size: buffer.len(), // pushing cuz battery dying :'D
            begin: &mut buffer.fi,
            end: buffer.last()
        };
    }
}