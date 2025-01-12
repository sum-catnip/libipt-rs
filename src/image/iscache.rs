use crate::error::{deref_ptresult_mut, ensure_ptok, extract_pterr, PtError, PtErrorCode};

use std::ffi::CString;
use std::ptr;

use crate::utils::name_ptr_to_option_string;
use libipt_sys::{
    pt_image_section_cache, pt_iscache_add_file, pt_iscache_alloc, pt_iscache_free,
    pt_iscache_name, pt_iscache_read, pt_iscache_set_limit,
};

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_isc_alloc() {
        SectionCache::new(None).unwrap();
        SectionCache::new(Some("yeet")).unwrap();
    }

    #[test]
    fn test_isc_name() {
        let i = SectionCache::new(None).unwrap();
        assert!(i.name().is_none());
        let i = SectionCache::new(Some("yeet")).unwrap();
        assert_eq!(i.name().unwrap(), "yeet");
    }

    #[test]
    fn test_isc_file() {
        let file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "testfiles", "garbage.txt"]
            .iter()
            .collect();
        println!("{:?}", file);

        SectionCache::new(None)
            .unwrap()
            .add_file(file.to_str().unwrap(), 5, 15, 0x1337)
            .unwrap();
    }

    #[test]
    fn test_isc_memsection() {
        let file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "testfiles", "garbage.txt"]
            .iter()
            .collect();

        println!("{:?}", file);
        let mut isc = SectionCache::new(None).unwrap();
        let isid = isc.add_file(file.to_str().unwrap(), 5, 24, 0x666).unwrap();

        let mut buf = [0; 20];
        isc.read(&mut buf, isid, 0x66A).unwrap();

        let expect = &fs::read(&file).unwrap()[9..29];
        assert_eq!(expect, buf);
    }

    #[test]
    fn test_isc_limit() {
        let mut isc = SectionCache::new(None).unwrap();
        isc.set_limit(111).unwrap();
        isc.set_limit(0).unwrap();
        isc.set_limit(std::u64::MAX).unwrap();
    }
}

/// A cache of traced image sections.
#[derive(Debug)]
pub struct SectionCache {
    pub(crate) inner: *mut pt_image_section_cache,
}
impl SectionCache {
    /// Allocate a traced memory image section cache.
    ///
    /// An optional @name may be given to the cache.
    /// The name string is copied.
    /// Returns a new traced memory image section cache on success
    pub fn new(name: Option<&str>) -> Result<Self, PtError> {
        deref_ptresult_mut(unsafe {
            match name {
                None => pt_iscache_alloc(ptr::null()),
                Some(n) => pt_iscache_alloc(
                    CString::new(n)
                        .map_err(|_| {
                            PtError::new(
                                PtErrorCode::Invalid,
                                "invalid @name string: contains null bytes",
                            )
                        })?
                        .as_ptr(),
                ),
            }
        })
        .map(|r| Self { inner: r })
    }

    /// Get the image section cache name.
    /// Name is optional
    pub fn name(&self) -> Option<String> {
        let name_ptr = unsafe { pt_iscache_name(self.inner) };
        name_ptr_to_option_string(name_ptr)
    }

    /// Add a new file section to the traced memory image section cache.
    ///
    /// Adds a new section consisting of @size bytes starting at @offset in @filename loaded at the virtual address @vaddr
    /// if @iscache does not already contain such a section.
    /// Returns an image section identifier (isid) uniquely identifying that section in @iscache.
    /// The section is silently truncated to match the size of @filename.
    /// Returns Invalid if @offset is too big.
    pub fn add_file(
        &mut self,
        filename: &str,
        offset: u64,
        size: u64,
        vaddr: u64,
    ) -> Result<u32, PtError> {
        let cfilename = CString::new(filename).map_err(|_| {
            PtError::new(
                PtErrorCode::Invalid,
                "Error converting filename to Cstring as it contains null bytes",
            )
        })?;

        extract_pterr(unsafe {
            pt_iscache_add_file(self.inner, cfilename.as_ptr(), offset, size, vaddr)
        })
    }

    /// Read memory from a cached file section
    ///
    /// Reads buffer.len bytes of memory starting at virtual address @vaddr in the section identified by @isid in @iscache into @buffer.
    /// The caller is responsible for allocating tye @buffer.
    /// The read request may be truncated if it crosses section boundaries or if buffer.len is getting too big.
    /// We support reading at least 4Kbyte in one chunk unless the read would cross a section boundary.
    /// Returns number of bytes read on success.
    /// Returns Nomap if @vaddr is not contained in section @isid.
    /// Returns BadImage if @iscache does not contain @isid.
    pub fn read(&mut self, buffer: &mut [u8], isid: u32, vaddr: u64) -> Result<u32, PtError> {
        extract_pterr(unsafe {
            pt_iscache_read(
                self.inner,
                buffer.as_mut_ptr(),
                buffer.len() as u64,
                isid as i32,
                vaddr,
            )
        })
    }

    /// Set the image section cache limit.
    ///
    /// Set the limit for a section cache in bytes.
    /// A non-zero limit will keep the least recently used sections mapped until the limit is reached.
    /// A limit of zero disables caching.
    pub fn set_limit(&mut self, limit: u64) -> Result<(), PtError> {
        ensure_ptok(unsafe { pt_iscache_set_limit(self.inner, limit) })
    }
}

impl Drop for SectionCache {
    fn drop(&mut self) {
        unsafe { pt_iscache_free(self.inner) }
    }
}
