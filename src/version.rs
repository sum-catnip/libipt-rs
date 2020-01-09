use std::ffi::CStr;
use libipt_sys::{
    pt_version,
    pt_library_version
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_version() {
        let v = Version::version();
        assert_ne!(v.major(), 0);
    }
}

/// The library version.
#[derive(Clone, Copy, Debug)]
pub struct Version(pt_version);
impl Version {
    /// Return the library version.
    pub fn version() -> Self {
        Version(unsafe { pt_library_version() })
    }

    /// Major version number.
    pub fn major(&self) -> u8 { self.0.major }

    /// Minor version number.
    pub fn minor(&self) -> u8 { self.0.minor }

    /// Patch level.
    pub fn patch(&self) -> u16 { self.0.patch }

    /// Build number.
    pub fn build(&self) -> u32 { self.0.build }

    /// Version extension.
    pub fn ext(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.ext) }.to_str().expect(
            concat!("failed to convert version ext to rust string. ",
                    "this is either a bug in libipt or the bindings")
        )
    }
}