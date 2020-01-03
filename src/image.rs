use crate::asid::Asid;
use crate::error::{ensure_ptok, PtError};
use std::mem;
use libipt_sys::{
    pt_image,
    pt_image_section_cache,
    pt_image_add_cached
};

pub struct Image(pub(crate) pt_image);
impl Image {
    /// Add a section from an image section cache.
    ///
    /// Add the section from @iscache identified by @isid in address space @asid.
    /// Existing sections that would overlap with the new section will be shrunk or split.
    /// Returns BadImage if @iscache does not contain @isid.
    pub fn add_cached(&mut self,
                      iscached: &mut SectionCache,
                      isid: i32,
                      asid: Asid) -> Result<(), PtError> {
        ensure_ptok(unsafe { pt_image_add_cached(
            &mut self.0, &mut iscached.0, isid, &asid.0
        )}).map(|_| ())
    }

    /// Add a new file section to the traced memory image.
    ///
    /// Adds @size bytes starting at @offset in @filename. The section is loaded at the virtual address @vaddr in the address space @asid.
    /// The @asid may be NULL or (partially) invalid. In that case only the valid fields are considered when comparing with other address-spaces. Use this when tracing a single process or when adding sections to all processes.
    /// The section is silently truncated to match the size of @filename.
    /// Existing sections that would overlap with the new section will be shrunk or split.
    /// Returns zero on success, a negative error code otherwise.
    /// Returns -pte_invalid if @image or @filename is NULL. Returns -pte_invalid if @offset is too big.

}

pub struct SectionCache(pub(crate) pt_image_section_cache);