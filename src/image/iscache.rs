use crate::error::{
    PtError,
    PtErrorCode,
    deref_ptresult,
    ensure_ptok,
    extract_pterr
};

use std::ffi::{CString, CStr};
use std::ptr;

use libipt_sys::{
    pt_image_section_cache,
    pt_iscache_add_file,
    pt_iscache_alloc,
    pt_iscache_name,
    pt_iscache_read,
    pt_iscache_set_limit
};

pub struct SectionCache(pub(crate) pt_image_section_cache);
impl SectionCache {
    /// Allocate a traced memory image section cache.
    ///
    /// An optional @name may be given to the cache. The name string is copied.
    /// Returns a new traced memory image section cache on success
    pub fn new(name: Option<&str>) -> Result<Self, PtError> {
        deref_ptresult(unsafe { pt_iscache_alloc(
            match name {
                None => ptr::null(),
                Some(n) =>
                    CString::new(n).map_err(|_| PtError::new(
                        PtErrorCode::Invalid,
                        "invalid @name string: contains null bytes"
                    ))?.as_ptr()
        })}).map(|s| SectionCache(*s))
    }

    /// Get the image section cache name.
    /// Name is optional
    pub fn name(&self) -> Option<&str> {
        deref_ptresult(unsafe { pt_iscache_name(&self.0) })
            .ok()
            .map(|s| unsafe { CStr::from_ptr(s) }.to_str().expect(
                concat!("failed converting name c-string.",
                        "this is a bug in either libipt or the bindings")
            ))
    }

    /// Add a new file section to the traced memory image section cache.
    ///
    /// Adds a new section consisting of @size bytes starting at @offset in @filename loaded at the virtual address @vaddr
    /// if @iscache does not already contain such a section.
    /// Returns an image section identifier (isid) uniquely identifying that section in @iscache.
    /// The section is silently truncated to match the size of @filename.
    /// Returns Invalid if @offset is too big.
    pub fn add_file(&mut self,
                    filename: &str,
                    offset: u64,
                    size: u64,
                    vaddr: u64) -> Result<u32, PtError> {
        let cfilename = CString::new(filename).map_err(|_| PtError::new(
            PtErrorCode::Invalid,
            "Error converting filename to Cstring as it contains null bytes")
        )?.as_ptr();

        extract_pterr(unsafe {
            pt_iscache_add_file(&mut self.0,
                                cfilename,
                                offset,
                                size,
                                vaddr)
        })
    }

    /// Read memory from a cached file section
    ///
    /// Reads buffer.len bytes of memory starting at virtual address @vaddr in the section identified by @isid in @iscache into @buffer.
    /// The caller is responsible for allocating tye @buffer.
    /// The read request may be truncated if it crosses section boundaries or if buffer.len is getting too big.
    /// We support reading at least 4Kbyte in one chunk unless the read would cross a section boundary.
    /// Returns a slice of read bytes into the @buffer.
    /// Returns Nomap if @vaddr is not contained in section @isid.
    /// Returns BadImage if @iscache does not contain @isid.
    pub fn read<'a>(&mut self,
                    buffer: &'a mut [u8],
                    isid: i32,
                    vaddr: u64) -> Result<&'a [u8], PtError> {
        let s = extract_pterr(unsafe {
            pt_iscache_read(&mut self.0,
                            buffer.as_mut_ptr(),
                            buffer.len() as u64,
                            isid,
                            vaddr)
        })?;
        Ok(&buffer[..s as usize])
    }

    /// Set the image section cache limit.
    ///
    /// Set the limit for a section cache in bytes.
    /// A non-zero limit will keep the least recently used sections mapped until the limit is reached.
    /// A limit of zero disables caching.
    pub fn set_limit(&mut self, limit: u64) -> Result<(), PtError> {
        ensure_ptok(unsafe { pt_iscache_set_limit(&mut self.0, limit) })
    }
}