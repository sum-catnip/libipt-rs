use libipt_sys::{pt_asid, pt_asid_no_cr3 as NO_CR3, pt_asid_no_vmcs as NO_VMCS};

/// An Intel PT address space identifier.
///
/// This identifies a particular address space when adding file sections or
/// when reading memory.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Asid(pub(crate) pt_asid);
impl Asid {
    #[inline]
    #[must_use]
    pub const fn new(cr3: Option<u64>, vmcs: Option<u64>) -> Self {
        let cr3_raw = match cr3 {
            Some(v) => v,
            None => NO_CR3,
        };
        let vmcs_raw = match vmcs {
            Some(v) => v,
            None => NO_VMCS,
        };

        Asid(pt_asid {
            size: size_of::<pt_asid>(),
            cr3: cr3_raw,
            vmcs: vmcs_raw,
        })
    }

    /// The CR3 value.
    #[inline]
    #[must_use]
    pub const fn cr3(self) -> Option<u64> {
        match self.0.cr3 {
            NO_CR3 => None,
            x => Some(x),
        }
    }

    /// The CR3 value.
    #[inline]
    pub fn set_cr3(&mut self, cr3: u64) {
        self.0.cr3 = cr3;
    }

    /// The VMCS Base address.
    #[inline]
    #[must_use]
    pub const fn vmcs(self) -> Option<u64> {
        match self.0.vmcs {
            NO_VMCS => None,
            x => Some(x),
        }
    }

    /// The VMCS Base address.
    #[inline]
    pub fn set_vmcs(&mut self, vmcs: u64) {
        self.0.vmcs = vmcs;
    }
}

impl Default for Asid {
    fn default() -> Self {
        Asid::new(None, None)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_asid_basic_values() {
        let mut asid = Asid::new(Some(0), Some(2));
        assert_eq!(asid.cr3(), Some(0));
        assert_eq!(asid.vmcs(), Some(2));

        // -1 because max would be NO_CR3
        asid.set_cr3(std::u64::MAX - 1);
        asid.set_vmcs(std::i64::MAX as u64);
        assert_eq!(asid.cr3(), Some(std::u64::MAX - 1));
        assert_eq!(asid.vmcs(), Some(std::i64::MAX as u64));
    }

    #[test]
    fn test_asid_default() {
        let asid: Asid = Default::default();
        assert_eq!(asid.cr3(), None);
        assert_eq!(asid.vmcs(), None);

        let asid: Asid = Asid::new(None, Some(1));
        assert_eq!(asid.cr3(), None);
        assert_eq!(asid.vmcs(), Some(1));

        let asid: Asid = Asid::new(Some(2), None);
        assert_eq!(asid.cr3(), Some(2));
        assert_eq!(asid.vmcs(), None);
    }

    #[test]
    fn test_asid_equal() {
        let asid: Asid = Default::default();
        let asid2: Asid = Default::default();
        assert_eq!(asid, asid2);

        let asid = Asid::new(Some(0), Some(666));
        let asid2 = Asid::new(Some(0), Some(666));
        assert_eq!(asid, asid2);

        let asid = Asid::new(Some(0), Some(1));
        let asid2 = Asid::new(Some(0), Some(2));
        assert_ne!(asid, asid2);

        let asid = Asid::new(None, Some(1));
        let asid2 = Asid::new(Some(0), Some(2));
        assert_ne!(asid, asid2);
    }

    #[test]
    fn test_asid_from() {
        let asid: Asid = Default::default();
        let raw = asid.0;
        assert_eq!(raw.cr3, NO_CR3);
        assert_eq!(raw.vmcs, NO_VMCS);

        let mut asid2 = Asid(raw);
        asid2.set_cr3(666);

        assert_eq!(asid.cr3(), None);
        assert_eq!(asid2.cr3(), Some(666));
        assert_eq!(raw.cr3, NO_CR3);
    }
}
