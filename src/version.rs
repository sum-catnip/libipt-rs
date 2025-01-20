use libipt_sys::{pt_library_version, pt_version};
use std::ffi::CStr;

/// The library version.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Version(pt_version);
impl Version {
    /// Return the libipt library version.
    #[must_use]
    pub fn current() -> Self {
        Version(unsafe { pt_library_version() })
    }

    /// Major version number.
    #[must_use]
    pub const fn major(&self) -> u8 {
        self.0.major
    }

    /// Minor version number.
    #[must_use]
    pub const fn minor(&self) -> u8 {
        self.0.minor
    }

    /// Patch level.
    #[must_use]
    pub const fn patch(&self) -> u16 {
        self.0.patch
    }

    /// Build number.
    #[must_use]
    pub const fn build(&self) -> u32 {
        self.0.build
    }

    /// Version extension.
    #[must_use]
    pub fn ext(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.ext) }
            .to_str()
            .expect(concat!(
                "failed to convert version ext to rust string. ",
                "this is either a bug in libipt or the bindings"
            ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_version() {
        let v = Version::current();
        assert_ne!(v.major(), 0);
    }
}
