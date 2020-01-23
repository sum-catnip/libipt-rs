use super::SectionCache;
use crate::asid::Asid;
use crate::error::{
    deref_ptresult,
    deref_ptresult_mut,
    ensure_ptok,
    extract_pterr,
    PtError,
    PtErrorCode
};
use std::ffi::{CStr, CString, c_void};
use std::ptr;
use std::mem;
use std::slice;
use libipt_sys::{
    pt_image,
    pt_image_add_cached,
    pt_image_add_file,
    pt_image_alloc,
    pt_image_copy,
    pt_image_free,
    pt_image_name,
    pt_image_remove_by_asid,
    pt_image_remove_by_filename,
    pt_image_set_callback,
    pt_asid
};

#[cfg(test)]
mod test {
    use super::*;

    // not much to test for in the unit tests
    // the integration tests should have way more stuffs

    #[test]
    fn test_img_alloc() {
        Image::new(None).unwrap();
        Image::new(Some("yeet")).unwrap();
    }

    #[test]
    fn test_img_name() {
        let i = Image::new(Some("yeet")).unwrap();
        assert_eq!(i.name().unwrap(), "yeet");
        let i = Image::new(None).unwrap();
        assert!(i.name().is_none());
    }
}

unsafe extern "C" fn read_callback(buffer: *mut u8,
                                   size: usize,
                                   asid: *const pt_asid,
                                   ip: u64,
                                   context: *mut c_void) -> i32 {
    let c: &mut &mut dyn FnMut(&mut [u8], u64, Asid) -> i32
        = mem::transmute(context);
    c(slice::from_raw_parts_mut(buffer, size), ip, Asid(*asid))
}

/// The traced memory image.
pub struct Image<'a> {
    // the wrapped inst
    pub(crate) inner: &'a mut pt_image,
    // do we need to free this instance on drop?
    dealloc: bool
}

impl<'a> Image<'a> {
    /// Allocate a traced memory image.
    /// An optional @name may be given to the image.
    /// The name string is copied.
    pub fn new(name: Option<&str>) -> Result<Self, PtError> {
        deref_ptresult_mut( unsafe { match name {
            None => pt_image_alloc(ptr::null()),
            Some(n) => pt_image_alloc(
                CString::new(n).map_err(|_| PtError::new(
                    PtErrorCode::Invalid,
                    "invalid @name string: contains null bytes"
                ))?.as_ptr())
        }}).map(|i| Image { inner: i, dealloc: true })
    }

    /// Get the image name.
    /// The name is optional.
    pub fn name(&self) -> Option<&str> {
        deref_ptresult( unsafe { pt_image_name(self.inner) })
            .ok()
            .map(|s| unsafe { CStr::from_ptr(s) }.to_str().expect(
                concat!("failed to convert c into rust string. ",
                        "this is a bug in either the bindings or libipt")
            ))
    }

    /// Remove all sections loaded into an address space.
    ///
    /// Removes all sections loaded into @asid.
    /// Specify the same @asid that was used for adding sections.
    /// Returns the number of removed sections on success.
    pub fn remove_by_asid(&mut self, asid: Asid) -> Result<u32, PtError> {
        extract_pterr( unsafe {
            pt_image_remove_by_asid(self.inner, &asid.0)
        })
    }

    /// Remove all sections loaded from a file.
    ///
    /// Removes all sections loaded from @filename from the address space @asid.
    /// Specify the same @asid that was used for adding sections from @filename.
    /// Returns the number of removed sections on success
    pub fn remove_by_filename(&mut self,
                              filename: &str,
                              asid: Asid)
                              -> Result<u32, PtError> {
        let cfilename = CString::new(filename).map_err(|_| PtError::new(
            PtErrorCode::Invalid,
            "Error converting filename to Cstring as it contains null bytes")
        )?.as_ptr();

        extract_pterr( unsafe {
            pt_image_remove_by_filename(self.inner,
                                        cfilename,
                                        &asid.0)
        })
    }

    /// Set the memory callback for the traced memory image.
    ///
    /// Sets @callback for reading memory.
    /// There can only be one callback at any time.
    /// A subsequent call will replace the previous callback.
    /// If @callback is None, the callback is removed.
    pub fn set_callback<F>(&mut self, callback: Option<F>) -> Result<(), PtError>
                           where F: FnMut(&mut [u8], u64, Asid) -> i32 {
        ensure_ptok(unsafe { match callback {
            None => pt_image_set_callback(self.inner, None, ptr::null_mut()),
            Some(mut cb) =>
                pt_image_set_callback(self.inner,
                                      Some(read_callback),
                                      &mut &mut cb as *mut _ as *mut c_void)
        }})
    }

    /// Copy an image.
    ///
    /// Adds all sections from @src.
    /// Sections that could not be added will be ignored.
    /// Returns the number of ignored sections on success.
    pub fn copy(&mut self, src: &Image) -> Result<u32, PtError> {
        extract_pterr( unsafe { pt_image_copy(self.inner, src.inner)})
    }

    /// Add a section from an image section cache.
    ///
    /// Add the section from @iscache identified by @isid in address space @asid.
    /// Existing sections that would overlap with the new section will be shrunk or split.
    /// Returns BadImage if @iscache does not contain @isid.
    pub fn add_cached(&mut self,
                      iscache: &mut SectionCache,
                      isid: i32,
                      asid: Asid) -> Result<(), PtError> {
        ensure_ptok( unsafe {
            pt_image_add_cached(self.inner, iscache.0, isid, &asid.0
        )})
    }

    /// Add a new file section to the traced memory image.
    ///
    /// Adds @size bytes starting at @offset in @filename.
    /// The section is loaded at the virtual address @vaddr in the address space @asid.
    /// The @asid may be None or (partially) invalid.
    /// In that case only the valid fields are considered when comparing with other address-spaces.
    /// Use this when tracing a single process or when adding sections to all processes.
    /// The section is silently truncated to match the size of @filename.
    /// Existing sections that would overlap with the new section will be shrunk or split.
    /// Returns Invalid if @offset is too big.
    /// Returns Invalid if @filename contains null bytes
    pub fn add_file(&mut self,
                    filename: &str,
                    offset: u64,
                    size: u64,
                    asid: Option<Asid>,
                    vaddr: u64) -> Result<(), PtError> {
        let cfilename = CString::new(filename).map_err(|_| PtError::new(
            PtErrorCode::Invalid,
            "Error converting filename to Cstring as it contains null bytes")
        )?.as_ptr();

        ensure_ptok( unsafe {
            pt_image_add_file(self.inner,
                              cfilename,
                              offset,
                              size,
                              match asid {
                                  Some(a) => &a.0,
                                  None => ptr::null()
                              }, vaddr)
        })}
}

impl<'a> From<&'a mut pt_image> for Image<'a> {
    fn from(img: &'a mut pt_image) -> Self {
        Image { inner: img, dealloc: false }
    }
}

impl<'a> Drop for Image<'a> {
    fn drop(&mut self) {
        if self.dealloc {
            unsafe { pt_image_free(self.inner) }
        }
    }
}