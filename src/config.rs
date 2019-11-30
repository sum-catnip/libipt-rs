use libipt_sys::{pt_config};

pub struct Config<'a> {
    _wrapped: pt_config,
    buffer: &'a[u8]
}

impl<'a> Config<'a> {
    /// the simples constructor
    /// offering none of the intelpt configuration options
    pub fn new(mut buffer: &'a [u8]) -> Config {
        // TODO error handling if buffer has no elements
        // would i really want to return Result<Config>?
        // seems a bit weird to have a failing ctor
        // maybe im just an oop slut
        let cfg = pt_config {
            size: buffer.len(), // pushing cuz battery dying :'D
            begin: buffer.first_mut(),
            end: buffer.last()
        };
    }
}